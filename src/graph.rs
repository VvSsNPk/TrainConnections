use std::cmp::Ordering;
use std::collections::{BinaryHeap, BTreeMap, HashMap};
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use chrono::{NaiveTime, TimeDelta};


pub struct Graph{
    pub graph: BTreeMap<Node,Vec<Edge>>,
}

impl Graph{
    pub fn new() -> Self{
        Self{
            graph: BTreeMap::new(),
        }
    }

    pub fn add_node(&mut self, node:Node){
        self.graph.entry(node).or_insert(Vec::new());
    }

    pub fn add_edge(&mut self, node: Node,edge: Edge){
        let x = self.graph.entry(node).or_insert(Vec::new());
        x.push(edge);

    }

    pub fn print_graph(&self){
        for i in &self.graph{
            print!("{} : ",i.0);
            for j in i.1{
                print!("-> ");
                print!("{},{},{},{},{}",j.node,j.location,j.train_no,j.arrival,j.departure);
            }
            println!()
        }
    }

    pub fn search_graph(&mut self, start: Node, goal: Node) -> Option<Pair>{
        let mut frontier = BinaryHeap::new();
        let mut distance_tracker = HashMap::new();
        for i in self.graph.keys(){
            distance_tracker.insert(i.clone(),usize::MAX);
        }
        let x = distance_tracker.entry(start.clone()).or_insert(0);
        *x =0;
        let mut pair = Pair::new(start.clone());
        let edge = Edge::new(start.clone(),0,String::from("start"),0,0,String::from("00:00:00"),String::new());
        pair.store.push(edge);
        pair.cost = 0;
        frontier.push(pair);
        while let Some(p) =frontier.pop(){
            if p.node == goal{
                return Some(p);
            }
            let mut x = p.cost;

            if x > distance_tracker.get(&p.node).copied().unwrap(){
                continue;
            }
            for edge in self.graph.get_mut(&p.node).unwrap(){
                let mut next = Pair::new(edge.node.clone());
                next.store.extend(p.store.clone());
                next.store.push(edge.clone());
                let mut x = next.sum_of_cost();
                next.cost = x;
                if x < distance_tracker.get(&edge.node).copied().unwrap(){
                    *distance_tracker.entry(edge.node.clone()).or_insert(0) = x;
                    frontier.push(next);
                }
            }
        }
    None

    }


    pub fn search_graph_three(&self,start: Node,time:String,end:Node) -> Option<Pair>{
        let mut frontier = BinaryHeap::new();
        let mut distance_tracker = HashMap::new();
        for i in self.graph.iter(){
            distance_tracker.insert(i.0.clone(),usize::MAX);
        }
        distance_tracker.insert(start.clone(),0);
        let children = self.graph.get(&start).unwrap();
        let mut store = Vec::new();
        for i in children{
            if i.node == start{
                store.push(i.clone());
            }
        }
        for i in store{
            for j in children{
                if i.train_no == j.train_no && i != *j{
                    let timer = NaiveTime::parse_from_str(time.as_str(),"%H:%M:%S").unwrap();
                    let timer1 = NaiveTime::parse_from_str(i.departure.as_str(),"%H:%M:%S").unwrap();
                    let timer2 = NaiveTime::parse_from_str(j.departure.as_str(),"%H:%M:%S").unwrap();
                    let mut diff = 0;
                    if timer1 > timer{
                        let d = timer1-timer;
                        diff += d.num_seconds();
                    }else{
                        let d = timer1-timer + TimeDelta::try_hours(24).unwrap();
                        diff += d.num_seconds();
                    }
                    if timer2 > timer1{
                        let d = timer2- timer1;
                        diff += d.num_seconds();
                    }else{
                        let d = timer2- timer1 + TimeDelta::try_hours(24).unwrap();
                        diff += d.num_seconds();
                    }
                    let mut pair = Pair::new(j.node.clone());
                    pair.store.push(i.clone());
                    pair.store.push(j.clone());
                    pair.cost = diff as usize;
                    frontier.push(pair);
                }
            }
        }
        while let Some(p) = frontier.pop(){
            if p.node == end{
                return Some(p)
            }
            if p.cost > distance_tracker.get(&p.node).copied().unwrap(){
                continue;
            }
            let edges = self.graph.get(&p.node).unwrap();
            let last = p.store.last().unwrap();
            let mut collector = Vec::new();
            for i in edges{
                if i.node == p.node {
                    collector.push(i);
                }
            }
            for i in collector{
                for j in edges{
                    if i.train_no == j.train_no && i != j{
                        if last.train_no == j.train_no{
                            let timer1 = NaiveTime::parse_from_str(i.departure.as_str(),"%H:%M:%S").unwrap();
                            let timer2 = NaiveTime::parse_from_str(j.departure.as_str(),"%H:%M:%S").unwrap();
                            let mut cost = p.cost;
                            if timer1 > timer2{
                                let diff = timer2 - timer1 + TimeDelta::try_hours(24).unwrap();
                                cost += diff.num_seconds() as usize;
                            }else{
                                let diff = timer2 - timer1;
                                cost += diff.num_seconds() as usize;
                            }
                            let mut pair = Pair::new(j.node.clone());
                            pair.cost = cost;
                            pair.store.extend(p.store.clone());
                            pair.store.push(j.clone());
                            if cost < distance_tracker.get(&j.node).copied().unwrap() {
                                *distance_tracker.entry(j.node.clone()).or_insert(0) = cost;
                                frontier.push(pair);
                            }
                        }else{
                            let timer1 = NaiveTime::parse_from_str(last.arrival.as_str(),"%H:%M:%S").unwrap();
                            let timer2 = NaiveTime::parse_from_str(i.departure.as_str(),"%H:%M:%S").unwrap();
                            let mut count = p.cost;
                            if timer1 < timer2{
                                let diff = timer2 - timer1;
                                if diff.num_seconds() <= 900{
                                    count += (diff + TimeDelta::try_hours(24).unwrap()).num_seconds() as usize;
                                }else{
                                    count += diff.num_seconds() as usize;
                                }
                            }else{
                                let diff = timer2 - timer1 + TimeDelta::try_hours(24).unwrap();
                                count += diff.num_seconds() as usize;
                            }
                            let time3 = NaiveTime::parse_from_str(j.departure.as_str(),"%H:%M:%S").unwrap();
                            if time3 < timer2{
                                let diff = time3 - timer2 + TimeDelta::try_hours(24).unwrap();
                                count += diff.num_seconds() as usize;
                            }else{
                                let diff = time3 - timer2;
                                count += diff.num_seconds() as usize;
                            }
                            let mut pair = Pair::new(j.node.clone());
                            pair.cost = count;
                            pair.store.extend(p.store.clone());
                            pair.store.push(i.clone());
                            pair.store.push(j.clone());
                            if count < distance_tracker.get(&j.node).copied().unwrap() {
                                *distance_tracker.entry(j.node.clone()).or_insert(0) = count;
                                frontier.push(pair);
                            }
                        }
                    }
                }
            }
        }
        None
    }

}

#[derive(Debug,Hash,PartialOrd, PartialEq,Ord, Eq,Clone)]
pub struct Edge {
    node: Node,
    pub cst : u32,
    pub cost: usize,
    pub train_no : String,
    pub location: u32,
    pub arrival: String,
    pub departure: String,
}

impl Edge{
    pub fn new(node: Node,cost: usize,train_no:String,location:u32,cst: u32,arrival:String,departure: String) -> Self{
        Self{
            node,
            cst,
            cost,
            train_no,
            location,
            arrival,
            departure
        }
    }

}

#[derive(Hash,PartialOrd, PartialEq,Ord,Eq,Debug,Clone)]
pub struct Node{
    station: String,
}

impl Node {
    pub fn new(station: String) -> Self{
        Self{
            station,
        }
    }
}

impl Display for Node{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.station)
    }
}

#[derive(Hash,Eq,PartialEq,Clone,Debug)]
pub struct Pair{
    node: Node,
    pub store: Vec<Edge>,
    pub cost: usize,
}

impl Pair {
    pub fn new(node: Node) -> Self{
        Self{
            node,
            store: Vec::new(),
            cost: 0

        }
    }

    pub fn new_again(node: Node,cost: usize) -> Self{
        Self{
            node,
            store: Vec::new(),
            cost
        }
    }

    pub fn sum_of_cost(&self) -> usize{
        self.store.iter().map(|s| s.cost).sum()
    }
}

impl PartialOrd for Pair{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair{
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
