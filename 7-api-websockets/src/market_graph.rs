use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;
use std::cell::RefCell;
use std::vec;

type Element = Rc<RefCell<Node>>;

struct Edge {
    price: f64,
    node: Element
}

struct Node {
    currency: String,
    neighbours: Vec<Rc<RefCell<Edge>>>
}

pub struct MarketGraph {
    nodes: HashMap<String, Element>,
    markets: HashMap<String, (Element, Element)>
}

impl MarketGraph {
    pub fn new() -> Self {
        MarketGraph { 
            nodes: HashMap::new(),
            markets: HashMap::new()
        }
    }

    pub fn has_currency(&self, currency: &String) -> bool {
        self.nodes.contains_key(currency)
    }

    pub fn add_currency(&mut self, currency: &String) {
        let node = Node {
            currency: currency.to_owned(),
            neighbours: vec![]
        };

        self.nodes.insert(currency.to_owned(), Rc::new(RefCell::new(node)));
    }

    pub fn create_market(&mut self, market: &String, first_currency: &String, second_currency: &String) {
        self.add_neighbour(first_currency, second_currency);
        self.add_neighbour(second_currency, first_currency);

        let first_node = self.nodes.get(first_currency).unwrap();
        let second_node = self.nodes.get(second_currency).unwrap();

        self.markets.insert(market.to_string(), (first_node.clone(), second_node.clone()));
    }

    fn add_neighbour(&mut self, first_currency: &String, second_currency: &String) {
        let node = self.nodes.get(first_currency).unwrap().clone();

        let mut contains = false;

        // TODO

        if !contains {
            let other_node = self.nodes.get(second_currency).unwrap().clone();

            let edge = Edge {
                price: 0.0,
                node: other_node
            };

            node.borrow_mut().neighbours.push(Rc::new(RefCell::new(edge)));
        }

        self.add_neighbour(second_currency, first_currency);
    }

    pub fn update_price(&mut self, market: &String, price: f64) {
        let market = self.markets.get(market)
            .unwrap();

        let first_node = market.0.clone();
        let second_node = market.1.clone();

        // TODO
    }

    pub fn get_price(&self, first_currency: &String, second_currency: &String) -> Option<f64> {
        // TODO:
        None
    }
}