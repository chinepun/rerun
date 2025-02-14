use ahash::HashSet;
use itertools::Itertools;
use nohash_hasher::IntMap;
use re_data_store::{EntityPath, EntityTree, InstancePath};
use re_data_ui::item_ui;
use re_log_types::EntityPathExpr;
use re_viewer_context::{DataQueryResult, SpaceViewId, ViewerContext};

use crate::{
    space_info::SpaceInfoCollection,
    space_view::SpaceViewBlueprint,
    space_view_heuristics::{
        compute_heuristic_context_for_entities, is_entity_processed_by_class,
        HeuristicFilterContextPerEntity,
    },
};

/// Window for adding/removing entities from a space view.
#[derive(Clone)]
pub struct SpaceViewEntityPicker {
    pub space_view_id: SpaceViewId,
}

impl SpaceViewEntityPicker {
    #[allow(clippy::unused_self)]
    pub fn ui(
        &mut self,
        ctx: &ViewerContext<'_>,
        ui: &egui::Ui,
        space_view: &mut SpaceViewBlueprint,
    ) -> bool {
        // This function fakes a modal window, since egui doesn't have them yet: https://github.com/emilk/egui/issues/686

        // In particular, we dim the background and close the window when the user clicks outside it
        let painter = egui::Painter::new(
            ui.ctx().clone(),
            egui::LayerId::new(egui::Order::PanelResizeLine, egui::Id::new("DimLayer")),
            egui::Rect::EVERYTHING,
        );
        painter.add(egui::Shape::rect_filled(
            ui.ctx().screen_rect(),
            egui::Rounding::ZERO,
            egui::Color32::from_black_alpha(128),
        ));

        // Close window using escape button.
        let mut open = ui.input(|i| !i.key_pressed(egui::Key::Escape));
        let title = "Add/remove Entities";

        let response = egui::Window::new(title)
            // TODO(andreas): Doesn't center properly. `pivot(Align2::CENTER_CENTER)` seems to be broken. Also, should reset every time
            .default_pos(ui.ctx().screen_rect().center())
            .collapsible(false)
            .default_height(640.0)
            .resizable(true)
            .frame(egui::Frame {
                fill: ui.visuals().panel_fill,
                inner_margin: re_ui::ReUi::view_padding().into(),
                ..Default::default()
            })
            // We do a custom title bar for better adhoc styling.
            // TODO(andreas): Ideally the default title bar would already adhere to that style
            .title_bar(false)
            .show(ui.ctx(), |ui| {
                title_bar(ctx.re_ui, ui, title, &mut open);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    add_entities_ui(ctx, ui, space_view);
                });
            });

        // Any click outside causes the window to close.
        let cursor_was_over_window = response
            .and_then(|response| {
                ui.input(|i| i.pointer.interact_pos())
                    .map(|interact_pos| response.response.rect.contains(interact_pos))
            })
            .unwrap_or(false);
        if !cursor_was_over_window && ui.input(|i| i.pointer.any_pressed()) {
            open = false;
        }

        open
    }
}

fn add_entities_ui(
    ctx: &ViewerContext<'_>,
    ui: &mut egui::Ui,
    space_view: &mut SpaceViewBlueprint,
) {
    let spaces_info = SpaceInfoCollection::new(ctx.store_db);
    let tree = &ctx.store_db.tree();
    let heuristic_context_per_entity = compute_heuristic_context_for_entities(ctx.store_db);
    // TODO(jleibs): Avoid clone
    let query_result = ctx.lookup_query_result(space_view.query_id()).clone();
    let inclusions: HashSet<EntityPathExpr> = space_view.inclusions().collect();
    let exclusions: HashSet<EntityPathExpr> = space_view.exclusions().collect();
    let entities_add_info = create_entity_add_info(
        ctx,
        tree,
        &heuristic_context_per_entity,
        space_view,
        &query_result,
        &spaces_info,
    );

    add_entities_tree_ui(
        ctx,
        ui,
        &tree.path.to_string(),
        tree,
        space_view,
        &query_result,
        &inclusions,
        &exclusions,
        &entities_add_info,
    );
}

#[allow(clippy::too_many_arguments)]
fn add_entities_tree_ui(
    ctx: &ViewerContext<'_>,
    ui: &mut egui::Ui,
    name: &str,
    tree: &EntityTree,
    space_view: &mut SpaceViewBlueprint,
    query_result: &DataQueryResult,
    inclusions: &HashSet<EntityPathExpr>,
    exclusions: &HashSet<EntityPathExpr>,
    entities_add_info: &IntMap<EntityPath, EntityAddInfo>,
) {
    if tree.is_leaf() {
        add_entities_line_ui(
            ctx,
            ui,
            &format!("🔹 {name}"),
            tree,
            space_view,
            query_result,
            inclusions,
            exclusions,
            entities_add_info,
        );
    } else {
        let level = tree.path.len();
        let default_open = space_view.space_origin.is_descendant_of(&tree.path)
            || tree.children.len() <= 3
            || level < 2;
        egui::collapsing_header::CollapsingState::load_with_default_open(
            ui.ctx(),
            ui.id().with(name),
            default_open,
        )
        .show_header(ui, |ui| {
            add_entities_line_ui(
                ctx,
                ui,
                name,
                tree,
                space_view,
                query_result,
                inclusions,
                exclusions,
                entities_add_info,
            );
        })
        .body(|ui| {
            for (path_comp, child_tree) in tree.children.iter().sorted_by_key(|(_, child_tree)| {
                // Put descendants of the space path always first
                let put_first = child_tree.path == space_view.space_origin
                    || child_tree.path.is_descendant_of(&space_view.space_origin);
                !put_first
            }) {
                add_entities_tree_ui(
                    ctx,
                    ui,
                    &path_comp.to_string(),
                    child_tree,
                    space_view,
                    query_result,
                    inclusions,
                    exclusions,
                    entities_add_info,
                );
            }
        });
    };
}

#[allow(clippy::too_many_arguments)]
fn add_entities_line_ui(
    ctx: &ViewerContext<'_>,
    ui: &mut egui::Ui,
    name: &str,
    entity_tree: &EntityTree,
    space_view: &mut SpaceViewBlueprint,
    query_result: &DataQueryResult,
    inclusions: &HashSet<EntityPathExpr>,
    exclusions: &HashSet<EntityPathExpr>,
    entities_add_info: &IntMap<EntityPath, EntityAddInfo>,
) {
    ui.horizontal(|ui| {
        let entity_path = &entity_tree.path;

        let add_info = entities_add_info.get(entity_path).unwrap();

        let included = query_result.contains_any(entity_path);

        // TODO(jleibs): Speed this up
        let excluded = exclusions.iter().any(|expr| match expr {
            EntityPathExpr::Exact(expr) => expr == entity_path,
            EntityPathExpr::Recursive(expr) => {
                expr == entity_path || entity_path.is_descendant_of(expr)
            }
        });

        ui.add_enabled_ui(add_info.can_add_self_or_descendant.is_compatible(), |ui| {
            let widget_text = if excluded {
                // TODO(jleibs): Better design-language for excluded.
                egui::RichText::new(name).italics()
            } else if entity_path == &space_view.space_origin {
                egui::RichText::new(name).strong()
            } else {
                egui::RichText::new(name)
            };
            let response = item_ui::instance_path_button_to(
                ctx,
                ui,
                Some(space_view.id),
                &InstancePath::entity_splat(entity_path.clone()),
                widget_text,
            );
            if query_result.contains_entity(entity_path) {
                response.highlight();
            }
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            // Reset Remove Button
            {
                let enabled = add_info.can_add_self_or_descendant.is_compatible()
                    && (exclusions.contains(&EntityPathExpr::Recursive(entity_path.clone()))
                        || inclusions.contains(&EntityPathExpr::Recursive(entity_path.clone())));

                ui.add_enabled_ui(enabled, |ui| {
                    let response = ctx.re_ui.small_icon_button(ui, &re_ui::icons::RESET);

                    if response.clicked() {
                        space_view.clear_entity_expression(
                            ctx,
                            &EntityPathExpr::Recursive(entity_tree.path.clone()),
                        );
                    }

                    if enabled {
                        if exclusions.contains(&EntityPathExpr::Recursive(entity_path.clone())) {
                            response.on_hover_text("Stop excluding this EntityPath.");
                        } else if inclusions
                            .contains(&EntityPathExpr::Recursive(entity_path.clone()))
                        {
                            response.on_hover_text("Stop including this EntityPath.");
                        }
                    }
                });
            }

            // Remove Button
            {
                let enabled = query_result.contains_any(&entity_tree.path)
                    && add_info.can_add_self_or_descendant.is_compatible()
                    && !exclusions.contains(&EntityPathExpr::Recursive(entity_path.clone()));

                ui.add_enabled_ui(enabled, |ui| {
                    let response = ctx.re_ui.small_icon_button(ui, &re_ui::icons::REMOVE);

                    if response.clicked() {
                        space_view.add_entity_exclusion(
                            ctx,
                            EntityPathExpr::Recursive(entity_tree.path.clone()),
                        );
                    }

                    if enabled {
                        response.on_hover_text(
                            "Exclude this Entity and all its descendants from the Space View",
                        );
                    }
                });
            }

            // Add Button
            {
                let enabled = !included
                    && !excluded
                    && add_info.can_add_self_or_descendant.is_compatible()
                    && !inclusions.contains(&EntityPathExpr::Recursive(entity_path.clone()));

                ui.add_enabled_ui(enabled, |ui| {
                    let response = ctx.re_ui.small_icon_button(ui, &re_ui::icons::ADD);

                    if response.clicked() {
                        space_view.add_entity_inclusion(
                            ctx,
                            EntityPathExpr::Recursive(entity_tree.path.clone()),
                        );
                    }

                    if enabled {
                        if add_info.can_add.is_compatible_and_missing() {
                            response.on_hover_text(
                                "Include this Entity and all its descendants in the Space View",
                            );
                        } else {
                            response
                                .on_hover_text("Add descendants of this Entity to the Space View");
                        }
                    } else if let CanAddToSpaceView::No { reason } = &add_info.can_add {
                        response.on_disabled_hover_text(reason);
                    }
                });
            }
        });
    });
}

/// Describes if an entity path can be added to a space view.
#[derive(Clone, PartialEq, Eq)]
enum CanAddToSpaceView {
    Compatible { already_added: bool },
    No { reason: String },
}

impl Default for CanAddToSpaceView {
    fn default() -> Self {
        Self::Compatible {
            already_added: false,
        }
    }
}

impl CanAddToSpaceView {
    /// Can be generally added but space view might already have this element.
    pub fn is_compatible(&self) -> bool {
        match self {
            CanAddToSpaceView::Compatible { .. } => true,
            CanAddToSpaceView::No { .. } => false,
        }
    }

    /// Can be added and spaceview doesn't have it already.
    pub fn is_compatible_and_missing(&self) -> bool {
        self == &CanAddToSpaceView::Compatible {
            already_added: false,
        }
    }

    pub fn join(&self, other: &CanAddToSpaceView) -> CanAddToSpaceView {
        match self {
            CanAddToSpaceView::Compatible { already_added } => {
                let already_added = if let CanAddToSpaceView::Compatible {
                    already_added: already_added_other,
                } = other
                {
                    *already_added && *already_added_other
                } else {
                    *already_added
                };
                CanAddToSpaceView::Compatible { already_added }
            }
            CanAddToSpaceView::No { .. } => other.clone(),
        }
    }
}

#[derive(Default)]
#[allow(dead_code)]
struct EntityAddInfo {
    can_add: CanAddToSpaceView,
    can_add_self_or_descendant: CanAddToSpaceView,
}

fn create_entity_add_info(
    ctx: &ViewerContext<'_>,
    tree: &EntityTree,
    heuristic_context_per_entity: &HeuristicFilterContextPerEntity,
    space_view: &SpaceViewBlueprint,
    query_result: &DataQueryResult,
    spaces_info: &SpaceInfoCollection,
) -> IntMap<EntityPath, EntityAddInfo> {
    let mut meta_data: IntMap<EntityPath, EntityAddInfo> = IntMap::default();

    tree.visit_children_recursively(&mut |entity_path| {
        let heuristic_context_per_entity = heuristic_context_per_entity.get(entity_path).copied().unwrap_or_default();
        let can_add: CanAddToSpaceView =
            if is_entity_processed_by_class(ctx, space_view.class_identifier(), entity_path, heuristic_context_per_entity, &ctx.current_query()) {
                match spaces_info.is_reachable_by_transform(entity_path, &space_view.space_origin) {
                    Ok(()) => CanAddToSpaceView::Compatible {
                        already_added: query_result.contains_any(entity_path),
                    },
                    Err(reason) => CanAddToSpaceView::No {
                        reason: reason.to_string(),
                    },
                }
            } else {
                CanAddToSpaceView::No {
                    reason: format!(
                        "Entity can't be displayed by this class of Space View ({}), since it doesn't match any archetype that the Space View can process.",
                        space_view.class_identifier()
                    ),
                }
            };

        if can_add.is_compatible() {
            // Mark parents aware that there is some descendant that is compatible
            let mut path = entity_path.clone();
            while let Some(parent) = path.parent() {
                let data = meta_data.entry(parent.clone()).or_default();
                data.can_add_self_or_descendant = data.can_add_self_or_descendant.join(&can_add);
                path = parent;
            }
        }

        let can_add_self_or_descendant = can_add.clone();
        meta_data.insert(
            entity_path.clone(),
            EntityAddInfo {
                can_add,
                can_add_self_or_descendant,
            },
        );
    });

    meta_data
}

fn title_bar(re_ui: &re_ui::ReUi, ui: &mut egui::Ui, title: &str, open: &mut bool) {
    ui.horizontal(|ui| {
        ui.strong(title);

        ui.add_space(16.0);

        let mut ui = ui.child_ui(
            ui.max_rect(),
            egui::Layout::right_to_left(egui::Align::Center),
        );
        if re_ui
            .small_icon_button(&mut ui, &re_ui::icons::CLOSE)
            .clicked()
        {
            *open = false;
        }
    });
    ui.separator();
}
