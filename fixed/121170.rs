use std::ops::{Add, Mul};

fn problematic_function<Space>(material_surface_element: Edge2dElement)
where
    DefaultAllocator: FiniteElementAllocator<DimU1, Space>,
{
    let _: Point2<f64> = material_surface_element.map_reference_coords().into();
}

impl<N, R, C> Allocator<N, R, C> for DefaultAllocator
where
    R: DimName,
    C: DimName,
    R::Value: Mul<C::Value>,
    Prod<R::Value, C::Value>:,
{
    type Buffer = ArrayStorage<N, R, C>;
}
impl<N, C> Allocator<N, Dynamic, C> for DefaultAllocator {}
impl DimName for DimU1 {}
impl DimName for DimU2 {}
impl<N, D> From<VectorN<N, D>> for Point<N, D> {}

impl FiniteElement<DimU2> for Edge2dElement {}

type Owned<N, R, C> = <DefaultAllocator as Allocator<N, R, C>>::Buffer;
type MatrixMN<N, R, C> = Matrix<N, R, C, Owned<N, R, C>>;
type VectorN<N, D> = MatrixMN<N, D, DimU1>;

type Point2<N> = Point<N, DimU2>;

struct B0;
struct B1;

struct DefaultAllocator;
struct Dynamic;
struct DimU1;
struct DimU2;
struct Matrix<N, R, C, S> {}
struct ArrayStorage<N, R, C> {}

struct Point<N, D> {}
struct Edge2dElement;

trait Allocator<Scalar, R, C = DimU1> {
    type Buffer;
}
trait DimName {}
trait FiniteElementAllocator<GeometryDim, NodalDim>:
    Allocator<f64, ()> + Allocator<f64, NodalDim>
{
}

trait FiniteElement<GeometryDim> {
    fn map_reference_coords(&self) -> VectorN<f64, GeometryDim>;
}
