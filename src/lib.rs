//! Clustering algorithms.
//!
//! ![A cluster](http://upload.wikimedia.org/wikipedia/commons/e/e1/Cassetteandfreehub.jpg)
//!
//! This crate provides generic implementations of clustering
//! algorithms, allowing them to work with any back-end "point
//! database" that implements the required operations, e.g. one might
//! be happy with using the naive collection `BruteScan` from this
//! crate, or go all out and implement a specialised R*-tree for
//! optimised performance.
//!
//! **Density-based clustering algorithms:**
//!
//! - DBSCAN (`Dbscan`)
//! - OPTICS (`Optics`)
//!
//! **Hierarchical clustering algorithms:**
//!
//! - Agglomerative or bottom-up clustering (`AgglomerativeClustering`)
//!   * _Single-linkage criterion:_ `NaiveBottomUp` using `SingleLinkage`, `SLINK`
//!   * _Complete-linkage criterion:_ `NaiveBottomUp` using `CompleteLinkage`, `CLINK`
//!
//! **Others:**
//!
//! - *k*-means (`Kmeans`)
//!
//! [Source](https://github.com/huonw/cogset).
//!
//! # Installation
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! cogset = "0.2"
//! ```

#![cfg_attr(all(test, feature = "unstable"), feature(test))]
#[cfg(all(test, feature = "unstable"))] extern crate test;
#[cfg(test)] extern crate rand;
#[cfg(test)] extern crate quickcheck;
#[cfg(test)] extern crate permutohedron;

extern crate order_stat;

#[cfg(all(test, feature = "unstable"))]
#[macro_use]
mod benches;
#[cfg(not(all(test, feature = "unstable")))]
macro_rules! make_benches {
    ($($_x: tt)*) => {}
}

mod dbscan;
pub use dbscan::Dbscan;

mod optics;
pub use optics::{Optics, OpticsDbscanClustering};

mod point;
pub use point::{Point, RegionQuery, Points, ListPoints, BruteScan, BruteScanNeighbours,
                Euclid, Euclidean};

mod kmeans;
pub use kmeans::{Kmeans, KmeansBuilder};

mod hierarchical;
pub use hierarchical::{AgglomerativeClustering, NaiveBottomUp, LinkageCriterion, SingleLinkage,
    CompleteLinkage, SLINK, CLINK, Dendrogram, Elements};
