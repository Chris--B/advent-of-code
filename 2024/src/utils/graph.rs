#![allow(unused)]

use crate::prelude::*;

use std::cmp::{Ord, Ordering, Reverse};
use std::hash::Hash;

pub trait Graph {
    type Vert: Copy + Eq + Hash + std::fmt::Debug;

    fn verts(&self) -> impl Iterator<Item = Self::Vert>;

    fn neighbors(&self, vert: &Self::Vert) -> impl Iterator<Item = Self::Vert> + 'static;

    fn edge_weight(&self, from: &Self::Vert, to: &Self::Vert) -> Option<i64>;

    fn distance_get(&self, vert: Self::Vert) -> Option<i64>;
    fn distance_set(&mut self, vert: Self::Vert, dist: i64);

    fn prev_set(&mut self, vert: Self::Vert, prev: Self::Vert);
}

/// A grid of '.' and '#', where '#' are always impassable.
pub struct AocGridGraph {
    pub map: Framebuffer<char>,
    pub dist: Framebuffer<i64>,

    // Each cell points to the next cell in the shortest path(s)
    pub prev: Framebuffer<Option<IVec2>>,
}

impl AocGridGraph {
    pub fn new(map: Framebuffer<char>) -> Self {
        let mut dist = Framebuffer::new_matching_size(&map);
        dist.clear(i64::MAX);

        let mut path = Framebuffer::new_matching_size(&map);

        Self {
            map,
            dist,
            prev: path,
        }
    }
}

impl AocGridGraph {
    pub fn shortest_path(&mut self, start: IVec2, end: IVec2) -> Option<Vec<IVec2>> {
        if let None = self.distance_get(end) {
            return None;
        }

        let mut path = vec![end];

        let mut i = 0;
        let mut curr = end;
        while let Some(prev) = self.prev[curr] {
            path.push(prev);
            i += 1;
            curr = prev;
        }

        path.reverse();
        Some(path)
    }
}

impl Graph for AocGridGraph {
    type Vert = IVec2;

    fn verts(&self) -> impl Iterator<Item = Self::Vert> {
        self.map.iter_coords().map(|(x, y)| IVec2::new(x, y))
    }

    fn neighbors(&self, vert: &Self::Vert) -> impl Iterator<Item = Self::Vert> + 'static {
        let neighbors: SmallVec<[IVec2; 4]> = vert
            .neighbors()
            .into_iter()
            .filter(|n| self.map[n] == '.')
            .collect();

        neighbors.into_iter()
    }

    fn edge_weight(&self, from: &Self::Vert, to: &Self::Vert) -> Option<i64> {
        let diff: IVec2 = (*to - *from).abs();

        if ((self.map[from] == '.') && (self.map[to] == '.'))
            && ((diff.x == 0 && diff.y == 1) || (diff.x == 1 && diff.y == 0))
        {
            Some(1)
        } else {
            None
        }
    }

    fn distance_get(&self, vert: Self::Vert) -> Option<i64> {
        if self.dist[vert] != i64::MAX {
            Some(self.dist[vert])
        } else {
            None
        }
    }

    fn distance_set(&mut self, vert: Self::Vert, dist: i64) {
        self.dist[vert] = dist;
    }

    fn prev_set(&mut self, vert: Self::Vert, prev: Self::Vert) {
        self.prev[vert] = Some(prev);
    }
}

fn remove_min<T: Hash + Eq + Copy>(queue: &mut VecDeque<T>, g: &impl Graph<Vert = T>) -> Option<T> {
    if let Some((idx, _)) = queue
        .iter()
        .enumerate() //
        .min_by_key(|(_i, &v)| -> i64 {
            // Use the graph to find the minimum distance vertex
            g.distance_get(v).unwrap_or(i64::MAX)
        })
    {
        queue.remove(idx)
    } else {
        None
    }
}

pub fn dijkstra<G: Graph>(g: &mut G, start: G::Vert, end: Option<G::Vert>) -> Option<i64> {
    g.distance_set(start, 0);
    dijkstra_resume(g, start, end)
}

pub fn dijkstra_resume<G: Graph>(g: &mut G, resume: G::Vert, end: Option<G::Vert>) -> Option<i64> {
    let mut queue: VecDeque<G::Vert> = VecDeque::new();
    queue.push_back(resume);

    while let Some(curr) = remove_min(&mut queue, g) {
        let dist = g
            .distance_get(curr)
            .unwrap_or_else(|| panic!("No distance for curr={curr:?}"));
        if dist == i64::MAX {
            break;
        }
        if let Some(end) = end {
            if curr == end {
                break;
            }
        }

        for next in g.neighbors(&curr) {
            let weight = g.edge_weight(&curr, &next).unwrap_or_else(|| {
                panic!("No edge weight? But neighbors() returned this: {curr:?} -> {next:?}")
            });
            assert!(weight >= 0, "Dijsktra's Algorithm does not work with zero or negative edge weights! g.edge_weight({curr:?}, {next:?}) == {weight}");

            let old_dist = g.distance_get(next).unwrap_or(i64::MAX);
            if dist + weight < old_dist {
                // Better path, use this one.
                g.distance_set(next, dist + weight);
                g.prev_set(next, curr);
                queue.push_back(next);
            }
        }
    }

    // if cfg!(test) {
    //     let verts = g.verts().collect_vec();
    //     for vert in verts {
    //         if let Some(dist) = g.distance_get(vert) {
    //             println!("{vert:?}: {dist} steps");
    //         } else {
    //             // println!("{vert:?}: Unreachable");
    //         }
    //     }
    // }

    if let Some(end) = end {
        g.distance_get(end)
    } else {
        None
    }
}
