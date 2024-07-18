use num::Zero;
use num_traits::Float;

pub trait ShapeProperties {
    type Scalar: Float;
    fn vertices(&self) -> Self::Scalar;
    fn area(&self) -> Self::Scalar;
}


pub trait ShapeMethods
where
    Self: ShapeProperties
{
    fn vertices_area(&self) -> Self::Scalar;
}

// Gives all variants the same size in memory
#[derive(Debug)]
pub enum Shape<T: Float + Zero> {
    Square { side: T },
    Rectangle { width: T, height: T },
    Triangle { base: T, height: T },
}

impl <T: Float + Zero> ShapeProperties for Shape<T> {
    type Scalar = T;
    fn vertices(&self) -> T {
        match self {
            &Self::Square { side: _ } => {
                T::from(4.0).unwrap()
            },
            &Self::Rectangle { width: _, height : _} => {
                T::from(4.0).unwrap()
            }
            &Self::Triangle { base: _, height: _ } => {
                T::from(3.0).unwrap()
            },
        }
    }

    // Rather than deriving can implement switch/if-else over same type
    // Violates:
    // - Polymorphism
    //     - our type's behaviour is more complex than our type system really knows.
    //       we're manually encoding behaviour of variants
    fn area(&self) -> T {
        match self {
            &Self::Square { side } => {
                side * side
            },
            &Self::Rectangle { width, height } => {
                width * height
            }
            &Self::Triangle { base, height } => {
               T::from( 0.5).unwrap() * base * height
            },
        }
    }
}

impl<T: Float + Zero> ShapeMethods for Shape<T> {
    fn vertices_area(&self) -> T {
       ( T::one() / T::one() + self.vertices()) * self.area()
    }
}


// Retain some kind of compile time polymorphism
// Broadly, anything that implements 'Area' is known by the implementer to compatible with this function
// Not as granular as implementing Area on each shape though ...
pub fn total_vertices_area<T: ShapeMethods>(shapes: &[T]) -> <T as ShapeProperties>::Scalar {

    // Compiler can now see exactly what is happening in loop, and how to utilise cache
    let acc = shapes
        .iter()
        .fold(<T::Scalar as Zero>::zero(), |acc, x| acc + x.vertices_area());
    acc
}
