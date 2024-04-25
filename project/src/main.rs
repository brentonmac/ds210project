use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::io::BufRead;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct Graph {
    n: usize,
    outedges: Vec<Vec<usize>>,
}

impl Graph {
    fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            self.outedges[*u].push(*v);
        }
    }

    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    fn create_directed(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Graph {
            n,
            outedges: vec![vec![]; n],
        };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }
}

type ListOfEdges = Vec<(usize, usize)>;

fn read_csv(filename: &str) -> Result<ListOfEdges, Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.trim().split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let v: usize = parts.next().unwrap().parse().unwrap();
        edges.push((u, v));
    }

    Ok(edges)
}

fn main() {
    let filename = "musae_ES_edges.csv"; // Change this to your CSV file name
    let edges = read_csv(filename).expect("Error reading CSV file");

    let n = edges.iter().map(|&(u, v)| usize::max(u, v)).max().unwrap() + 1;

    let graph = Graph::create_directed(n, &edges);

    println!("{:?}", graph);
}
