use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Rc;
use std::cell::RefCell;
use std::vec;

use crate::binance::BinanceClient;

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

        for neighbour in &(*node).borrow().neighbours {
            let other_node: Element = (*neighbour.clone())
                .borrow()
                .node
                .clone();

            if (*other_node).borrow().currency.eq(second_currency) {
                contains = true;
                break;
            }
        }

        if !contains {
            let other_node = self.nodes.get(second_currency).unwrap().clone();

            let edge = Edge {
                price: 0.0,
                node: other_node
            };

            node.borrow_mut().neighbours.push(Rc::new(RefCell::new(edge)));

            self.add_neighbour(second_currency, first_currency);
        }
    }

    pub fn update_price(&mut self, market: &String, price: f64) {
        let market = self.markets.get(market)
            .unwrap();

        let first_node = market.0.clone();
        let second_node = market.1.clone();

        self.update_edge(first_node.clone(), second_node.clone(), price);
        self.update_edge(second_node.clone(), first_node.clone(), 1.0 / price);
    }

    fn update_edge(&mut self, first_node: Element, second_node: Element, value: f64) {
        for neighbour in &(*first_node).borrow().neighbours {
            let other_node = (*neighbour.clone()).borrow().node.clone();

            if (*other_node).borrow().currency.eq(&(*second_node).borrow().currency) {
                neighbour.clone().borrow_mut().price = value;
                break;
            }
        }
    }

    pub fn get_price(&self, first_currency: &String, second_currency: &String) -> Option<f64> {
        let initial_node = self.nodes.get(first_currency).unwrap().clone();
        let mut stack = vec![initial_node.clone()];
        let mut visited = HashSet::<String>::new();

        let mut prices: Vec<f64> = vec![1.0];

        let mut current = initial_node.clone();
        while !stack.is_empty() {
            current = stack.pop().unwrap();

            if (*current).borrow().currency.eq(second_currency) {
                break;
            }

            let current_price = prices.pop().unwrap();

            for neighbour in &(*current).borrow().neighbours {
                let edge = neighbour.clone();

                let other_node = (*neighbour.clone())
                    .borrow()
                    .node
                    .clone();
                    
                let currency = &(*other_node).borrow().currency;
                if !visited.contains(currency) {
                    stack.push(other_node.clone());
                    prices.push(current_price * (*edge).borrow().price);
                    visited.insert(currency.to_string());
                }
            }
        }

        if (*current).borrow().currency.eq(second_currency) {
            Some(prices.pop().unwrap())
        } else{
            None
        }
    }
}

// wow, that's dangerous (https://doc.rust-lang.org/nomicon/send-and-sync.html)
// we do not care about inconsistent values
unsafe impl Sync for BinanceClient {}
unsafe impl Send for BinanceClient {}

mod tests {
    use super::MarketGraph;

    #[test]
    fn create_graph() {
        let mut graph = MarketGraph::new();

        let currency = "BTC".to_owned();
        assert!(!graph.has_currency(&currency));

        graph.add_currency(&currency);

        assert!(graph.has_currency(&currency));
    }

    #[test]
    fn create_market() {
        let mut graph = MarketGraph::new();
        let market = "BTCUSDT".to_owned();
        let first_currency = "BTC".to_owned();
        let second_currency = "USDT".to_owned();

        graph.add_currency(&first_currency);
        graph.add_currency(&second_currency);
        graph.create_market(&market, &first_currency, &second_currency);

        let price = graph.get_price(&first_currency, &second_currency).unwrap();
        assert_eq!(price, 0.0);

        graph.update_price(&market, 1.0);

        let price2 = graph.get_price(&first_currency, &second_currency).unwrap();
        assert_eq!(price2, 1.0);
    }
}