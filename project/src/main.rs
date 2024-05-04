mod stats; // importing my 2 other files
mod popular;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

type Vertex = usize; // imported from lecture - defining types of the following types
type ListOfEdges = Vec<(Vertex,Vertex)>;
type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
struct Graph { // imported from lecture
    n: usize, 
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
    fn add_directed_edges(&mut self, // imported from lecture - adds directed edges to an existing graph
                          edges:&ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }
    fn sort_graph_lists(&mut self) { // imported from lecture - sorts the graph list
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    fn create_directed(n:usize,edges:&ListOfEdges) // imported from lecture - creates a directed graph
                                            -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }
    
    fn create_undirected(n:usize,edges:&ListOfEdges) // imported from lecture - creates an undirected graph
                                            -> Graph {
        let mut g = Self::create_directed(n,edges);
        g.add_directed_edges(&reverse_edges(edges));
        g.sort_graph_lists();
        g                                        
    }
    fn node_degrees(&self) -> Vec<usize> { // returns a vector of node degrees from a graph
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
    let celebs = popular::celeb(popularity);
    println!("There are {} vertices in the data.", graph.n);
    println!("The edges that are celebrities (7 on popularity scale) are {:?}.", celebs);
    println!("There are {} celebrities out of {} people.", celebs.len(), graph.n);
    println!("The mean of friendships every twitch user has is approximately {}.", mean.round());
    println!("The standard deviation for twitch user friendships is approximately {}.", stdev.round());
    println!("Minimum: {} | Q1: {} | Median: {} | Q3: {} | Maximum: {}", desc[0], desc[1], desc[2], desc[3], desc[4]);
    Ok(())
}

mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn test_reverse_edges() {
        let edges = vec![(1, 2), (2, 3), (3, 4)];
        let reversed_edges = reverse_edges(&edges);
        assert_eq!(reversed_edges, vec![(2, 1), (3, 2), (4, 3)]);
    }

    #[test]
    fn test_add_directed_edges() {
        let mut graph = Graph {
            n: 4,
            outedges: vec![vec![], vec![], vec![], vec![]],
        };
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        graph.add_directed_edges(&edges);
        assert_eq!(graph.outedges, vec![vec![1], vec![2], vec![3], vec![]]);
    }

    #[test]
    fn test_sort_graph_lists() {
        let mut graph = Graph {
            n: 4,
            outedges: vec![vec![3, 1], vec![4,1], vec![], vec![2]],
        };
        graph.sort_graph_lists();
        assert_eq!(graph.outedges, vec![vec![1, 3], vec![1,4], vec![], vec![2]]);
    }

    #[test]
    fn test_create_directed() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = Graph::create_directed(4, &edges);
        assert_eq!(
            graph.outedges,
            vec![vec![1], vec![2], vec![3], vec![]]
        );
    }

    #[test]
    fn test_node_degrees() {
        let graph = Graph {
            n: 4,
            outedges: vec![vec![1, 3], vec![2], vec![3], vec![4]],
        };
        let degrees = graph.node_degrees();
        assert_eq!(degrees, vec![2, 1, 1, 1]);
    }

    #[test]
    fn test_descriptive_stats() {
        let mut stats_struct = stats::Stats::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let desc = stats::Stats::descriptive_stats(&mut stats_struct);
        assert_eq!(desc, vec![1.0, 3.0, 5.5, 8.0, 10.0]);
    }

    #[test]
    fn test_mean() {
        let stats_struct = stats::Stats::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let mean = stats::Stats::mean(&stats_struct);
        assert_eq!(mean, 5.5);
    }

    #[test]
    fn test_stdev() {
        let stats_struct = stats::Stats::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let stdev = stats::Stats::stdev(&stats_struct);
        assert_eq!(stdev, 2.8722813232690143);
    }

    #[test]
    fn test_zscores() {
        let stats_struct = stats::Stats::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let zscores = stats::Stats::zscores(&stats_struct);
        assert_eq!(
            zscores,
            vec![-1.5666989036012806,-1.2185435916898848,-0.8703882797784892,-0.5222329678670935,-0.17407765595569785,0.17407765595569785,
                0.5222329678670935,0.8703882797784892,1.2185435916898848,1.5666989036012806]);
    }

    #[test]
    fn test_popularity_scale() {
        let zscores = vec![
            -1.5666989036012806,
            -1.2185435916898848,
            -0.8703882797784892,
            -0.5222329678670935,
            -0.17407765595569785,
            0.17407765595569785,
            0.5222329678670935,
            0.8703882797784892,
            1.2185435916898848,
            1.5666989036012806,
        ];
        let popularity = popular::popularity_scale(zscores);
        assert_eq!(popularity, vec![2, 2, 3, 3, 3, 4, 4, 4, 5, 5]);
    }

    #[test]
    fn test_celeb() {
        let pop_scale = vec![1, 1, 1, 2, 3, 4, 4, 5, 5, 7];
        let celebs = popular::celeb(pop_scale);
        assert_eq!(celebs, vec![9]);
    }
}