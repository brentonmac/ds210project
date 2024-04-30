mod stats;
mod popular;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

type Vertex = usize;
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct Graph {
    n: usize, // vertex labels in {0,...,n-1}
    outedges: AdjacencyLists,
}

// reverse direction of edges on a list
fn reverse_edges(list:&ListOfEdges)
        -> ListOfEdges {
    let mut new_list = vec![];
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    new_list
}

impl Graph {
    fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,edges:&ListOfEdges)
                                            -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    
    fn create_undirected(n:usize,edges:&ListOfEdges) // will have double he amount of edges (1 to 2 and 2 to 1)
                                            -> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g                                        
    }
    fn node_degrees(&self) -> Vec<usize> {
        let mut degrees = vec![0; self.n];
        for (i, l) in self.outedges.iter().enumerate() {
            degrees[i] = l.len();
        }
        degrees
    }
}
fn read_csv(file_path: &str) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
    // converts a csv file with 2 values in each row into a vector of tuples where the tuple is (val1, val2)
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().from_reader(file);
    let mut tuples = Vec::new();

    for result in reader.records() {
        let record = result?;
        let first = record[0].trim().parse::<usize>()?;
        let second = record[1].trim().parse::<usize>()?;
        tuples.push((first, second));
    }

    Ok(tuples)
}

fn main() -> Result<(), Box<dyn Error>>{
    let file_path = "musae_ES_edges.csv";
    let tuples = read_csv(file_path)?;
    let Some(max_val) = tuples.iter().max_by_key(|&(val1, _)| val1) else {todo!{}};
    let graph = Graph::create_undirected(max_val.0 + 1, &tuples);
    let node_deg = Graph::node_degrees(&graph);
    let mut stats_struct = stats::Stats::new(node_deg);
    let mean = stats::Stats::mean(&stats_struct);
    let stdev = stats::Stats::stdev(&stats_struct);
    let desc = stats::Stats::descriptive_stats(&mut stats_struct);
    let z_score = stats::Stats::zscores(&stats_struct);
    let popularity = popular::popularity_scale(z_score);
    let mut celebs = vec![];
    for i in 0..popularity.len() {
        if popularity[i] == 7 {
            celebs.push(i);
        }
    } 
    println!("The edges of the celebrities (7 on popularity scale) are {:?}.", celebs);
    println!("The average number of friendships every twitch user has is approximately {}.", mean.round());
    println!("The standard deviation for twitch user friendships is approximately {}.", stdev.round());
    println!("Minimum: {} | Q1: {} | Median: {} | Q3: {} | Maximum: {}", desc[0], desc[1], desc[2], desc[3], desc[4]);
    Ok(())
}