use crate::characteristics::{AntiSqrt, Attitude, Sqrt};
use crate::metrics::Distance;
use crate::norms::{BulkNorm, UnitizedNorm, WeightNorm};
use crate::products::dot::{AntiDot, Dot};
use crate::products::exterior::{Meet, Wedge};
use crate::products::geometric::WedgeDot;
use crate::unitize::Unitize;
use crate::{FlatPoint, Horizon, Infinity, Origin, RoundPoint, RoundPointOnOrigin};
use projective_ga::Unit;
use std::ops::Add;

#[test]
fn round_point_distances() {
    // TODO in order to better understand the "e1, e2, e3" object (tentatively called
    //  "RoundPointBulk"), I want to better understand the meaning of distances between
    //  RoundPoints, particularly RoundPoints with imaginary radius. If you consider
    //  a RoundPointOnOrigin, and then bring its e4 component to zero, you can see it
    //  approaches a point at infinity with an imaginary radius that stays at the origin.
    //  Thus it creates something like a PlaneAtOrigin. This also makes sense that a
    //  RoundPointBulk is commonly found as a WeightDual to various Planes. But if it's not
    //  a plane overtly.... then what is it? If we focus on the center of the point, it is like
    //  a "PlaneNormalVector" or something.. but as we know, the radial leash of a RoundPoint
    //  is one of it's most defining features. So it seems inappropriate to ignore. The e1,e2,e3
    //  elements certainly specify a direction, but if you want to be zero-distance from the
    //  object, you need to be near a PlaneAtOrigin, not near infinity. I guess.... this gets at
    //  the heart of a big and very interesting question. What is the fundamental authentic
    //  difference between a Sphere and RoundPoint? Because a plane is a sphere, and the sphere
    //  IS its surface, so a PlaneAtOrigin IS its surface. But a RoundPointBulk is zero distance
    //  away from some PlaneAtOrigin while NOT being its surface... it's weird.

    // TODO new problem.... RoundPoints don't have very many distance implementations.
    // TODO FlatPoints are same situation. Only have distance with Circles?

    // RoundPoint with real radius
    // 3, 0, 0 (radius of real 3)
    let a = RoundPoint::new(3.0, 0.0, 0.0, 1.0, 4.5);
    // If you do the above with a radius of imaginary 3, you get an imaginary distance for all results.

    // RoundPoints with zero radius

    // TODO none of the distances measured here care about the e5 parameter
    //  So the distance formula is measuring the distance between the centers of each
    //  RoundPoint, and not considering the radius.

    let b = RoundPoint::new(-3.0, 0.0, 0.0, 1.0, 4.5);
    let c = RoundPoint::new(-2.0, 0.0, 0.0, 1.0, 2.0);
    let d = RoundPoint::new(-1.0, 0.0, 0.0, 1.0, 0.5);
    let e = RoundPoint::new(0.0, 0.0, 0.0, 1.0, 0.0);
    let f = RoundPoint::new(1.0, 0.0, 0.0, 1.0, 0.5);
    let g = RoundPoint::new(2.0, 0.0, 0.0, 1.0, 2.0);
    let h = RoundPoint::new(3.0, 0.0, 0.0, 1.0, 4.5);
    let i = RoundPoint::new(4.0, 0.0, 0.0, 1.0, 8.0);
    let j = RoundPoint::new(5.0, 0.0, 0.0, 1.0, 12.5);
    let k = RoundPoint::new(6.0, 0.0, 0.0, 1.0, 18.0);

    for some_point in vec![b, c, d, e, f, g, h, i, j, k] {
        let diff = a - some_point;
        let distance = diff.dot(diff).sqrt();
        // println!("RoundPoint distance is {distance:?}");
    }

    // 3, 0, 0
    let a = FlatPoint::new(3.0, 0.0, 0.0, 1.0);

    let b = FlatPoint::new(-3.0, 0.0, 0.0, 1.0);
    let c = FlatPoint::new(-2.0, 0.0, 0.0, 1.0);
    let d = FlatPoint::new(-1.0, 0.0, 0.0, 1.0);
    let e = FlatPoint::new(0.0, 0.0, 0.0, 1.0);
    let f = FlatPoint::new(1.0, 0.0, 0.0, 1.0);
    let g = FlatPoint::new(2.0, 0.0, 0.0, 1.0);
    let h = FlatPoint::new(3.0, 0.0, 0.0, 1.0);
    let i = FlatPoint::new(4.0, 0.0, 0.0, 1.0);
    let j = FlatPoint::new(5.0, 0.0, 0.0, 1.0);
    let k = FlatPoint::new(6.0, 0.0, 0.0, 1.0);

    for mut some_point in vec![b, c, d, e, f, g, h, i, j, k] {
        // TODO.... apparently.... FlatPoints have an imaginary weight norm?
        //  FlatPoint.anti_dot(FlatPoint) is negative, which then we have to take square root
        //  to find the weight norm.... So any attempt to unitize (requiring division by weight norm)
        //  results in NaN...
        println!("some_point before: {some_point:?}");
        some_point = some_point.unitize();
        println!("some_point after: {some_point:?}");

        let round_a = some_point.meet(Horizon::unit());
        println!("Conversion of some_point: {round_a:?}");
        // let distance = a.distance(some_point).unitized_norm();
        // println!("FlatPoint distance is {distance}");
    }
}

fn distance_speculation() {
    // assume we can calculate distance between two RoundPoints
    // assume we can calculate distance between two Spheres
    // Assume we can calculate distance between RoundPoint and Sphere

    // The remaining combinations necessary...
    // - Dipole and RoundPoint
    // - Dipole and Dipole
    // - Dipole and Circle
    // - Dipole and Sphere
    // - Circle and RoundPoint
    // - Circle and Circle
    // - Circle and Sphere

    // So the strategy is this... use geometric operations to create a RoundPoint of zero radius
    // on the object that is closest to the other object being compared. If we do this from
    // one side, then that solves the following:
    // - Dipole and RoundPoint
    // - Dipole and Sphere
    // - Circle and RoundPoint
    // - Circle and Sphere

    // The above has the slightly awkward caveat of imaginary radius objects. So watch out for that.

    // After that however, we still have these remaining cases that can get awkward if they are
    // parallel or skew.
    // - Dipole and Dipole
    // - Dipole and Circle
    // - Circle and Circle

    // Anyway... more on creating a RoundPoint on a Dipole|Circle closest to RoundPoint|Sphere...
    // Take the RoundPoint|Sphere and take its center (which is itself a RoundPoint).
    // Force the Center (RoundPoint) radius to zero.
    // Then take the carrier of the other Dipole|Circle (which will be a Line|Plane), and
    // orthogonally project the earlier mentioned Center (RoundPoint) onto it.
    // Then orthogonally project the Center (RoundPoint) again, but this time onto the Container of
    // the Dipole|Circle. In fact... you might not need the Container at all... assuming these
    // impls actually work and don't have anything funky about them:
    // - impl ProjectOrthogonallyOnto<Dipole> for RoundPoint
    // - impl ProjectOrthogonallyOnto<Circle> for RoundPoint
    // Even better if those projections work accurately for Dipole|Circle of imaginary radius.

    // So what about the potentially skew/parallel cases?
    // Assuming for starters that these round objects are really round, and not infinite radius flat.
    // You can still try the above technique and see what happens. Each dipole has a center, each
    // circle has a center. Consecutive orthogonal projections, and bam you've got closest points.
    // But then what about the parallel/skew case?
    // Let's start with parallel circles because there is fewer degrees of freedom for degenerate
    // cases like this. There's only one way for circles to be parallel, but lots of ways for
    // dipoles to be parallel or skew.

    // Well... huh... when I first started attempting to create this technique, I was imagining
    // somehow creating a RoundPoint by meeting the carriers of both sides... but it's actually
    // kind of hard to figure out how to create a RoundPoint that way, and so then I suggested
    // using the center operation... but meeting parallel carriers goes to infinity, which is the
    // awkward part, but orthogonally projecting a's center onto b's carrier is not awkward at all.
    // For parallel circles, it works quite easily in fact. Still though... it's not perfect...

    // Imagine you have two concentric circles. One has twice the radius of the other.
    // They are on the xy plane. now take the smaller concentric circle, and translate it along the
    // z axis. And then finally, translate it slightly on the x axis (so the circles are no longer
    // concentric). Viewed from z=infinity, one circle should look like it's inside the other circle.
    // So.... now we try to find the distance between them. Take the center of one of the circles,
    // and project it onto the other's carrier, and then project onto the object. This gives us
    // the closest point on the bigger circle, but the furthest point on the smaller circle.
    // So how do we fix that?
    // Well... I have a hunch... since we're taking the centers anyway... there might be some
    // insight or manipulating we can do with the relative positioning and relative radii of the
    // two centers.

    // I'm really aching to see some 3d visualizations. Maybe it's time to start
    // developing a "CGA Playground" demo. Even without all the fundamentals (like Distance)
    // figured out yet.
}
