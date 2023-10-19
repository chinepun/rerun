# DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/python.rs
# Based on "crates/re_types/definitions/rerun/archetypes/pinhole.fbs".

# You can extend this class by creating a "PinholeExt" class in "pinhole_ext.py".

from __future__ import annotations

from attrs import define, field

from .. import components
from .._baseclasses import Archetype
from .pinhole_ext import PinholeExt

__all__ = ["Pinhole"]


@define(str=False, repr=False, init=False)
class Pinhole(PinholeExt, Archetype):
    """
    **Archetype**: Camera perspective projection (a.k.a. intrinsics).

    Example
    -------
    ### Simple Pinhole Camera:
    ```python
    import numpy as np
    import rerun as rr

    rr.init("rerun_example_pinhole", spawn=True)
    rng = np.random.default_rng(12345)

    image = rng.uniform(0, 255, size=[3, 3, 3])
    rr.log("world/image", rr.Pinhole(focal_length=3, width=3, height=3))
    rr.log("world/image", rr.Image(image))
    ```
    <center>
    <picture>
      <source media="(max-width: 480px)" srcset="https://static.rerun.io/pinhole_simple/9af9441a94bcd9fd54e1fea44fb0c59ff381a7f2/480w.png">
      <source media="(max-width: 768px)" srcset="https://static.rerun.io/pinhole_simple/9af9441a94bcd9fd54e1fea44fb0c59ff381a7f2/768w.png">
      <source media="(max-width: 1024px)" srcset="https://static.rerun.io/pinhole_simple/9af9441a94bcd9fd54e1fea44fb0c59ff381a7f2/1024w.png">
      <source media="(max-width: 1200px)" srcset="https://static.rerun.io/pinhole_simple/9af9441a94bcd9fd54e1fea44fb0c59ff381a7f2/1200w.png">
      <img src="https://static.rerun.io/pinhole_simple/9af9441a94bcd9fd54e1fea44fb0c59ff381a7f2/full.png" width="640">
    </picture>
    </center>
    """

    # __init__ can be found in pinhole_ext.py

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            image_from_camera=None,  # type: ignore[arg-type]
            resolution=None,  # type: ignore[arg-type]
            camera_xyz=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> Pinhole:
        """Produce an empty Pinhole, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    image_from_camera: components.PinholeProjectionBatch = field(
        metadata={"component": "required"},
        converter=components.PinholeProjectionBatch._required,  # type: ignore[misc]
    )
    # Camera projection, from image coordinates to view coordinates.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    resolution: components.ResolutionBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.ResolutionBatch._optional,  # type: ignore[misc]
    )
    # Pixel resolution (usually integers) of child image space. Width and height.
    #
    # Example:
    # ```text
    # [1920.0, 1440.0]
    # ```
    #
    # `image_from_camera` project onto the space spanned by `(0,0)` and `resolution - 1`.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    camera_xyz: components.ViewCoordinatesBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.ViewCoordinatesBatch._optional,  # type: ignore[misc]
    )
    # Sets the view coordinates for the camera.
    #
    # All common values are available as constants on the `components.ViewCoordinates` class.
    #
    # The default is `ViewCoordinates::RDF`, i.e. X=Right, Y=Down, Z=Forward, and this is also the recommended setting.
    # This means that the camera frustum will point along the positive Z axis of the parent space,
    # and the cameras "up" direction will be along the negative Y axis of the parent space.
    #
    # The camera frustum will point whichever axis is set to `F` (or the opposite of `B`).
    # When logging a depth image under this entity, this is the direction the point cloud will be projected.
    # With `RDF`, the default forward is +Z.
    #
    # The frustum's "up" direction will be whichever axis is set to `U` (or the opposite of `D`).
    # This will match the negative Y direction of pixel space (all images are assumed to have xyz=RDF).
    # With `RDF`, the default is up is -Y.
    #
    # The frustum's "right" direction will be whichever axis is set to `R` (or the opposite of `L`).
    # This will match the positive X direction of pixel space (all images are assumed to have xyz=RDF).
    # With `RDF`, the default right is +x.
    #
    # Other common formats are `RUB` (X=Right, Y=Up, Z=Back) and `FLU` (X=Forward, Y=Left, Z=Up).
    #
    # NOTE: setting this to something else than `RDF` (the default) will change the orientation of the camera frustum,
    # and make the pinhole matrix not match up with the coordinate system of the pinhole entity.
    #
    # The pinhole matrix (the `image_from_camera` argument) always project along the third (Z) axis,
    # but will be re-oriented to project along the forward axis of the `camera_xyz` argument.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__
