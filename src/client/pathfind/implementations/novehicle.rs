use std::collections::HashMap;

use crate::client::pathfind::context::{GlobalContext, MoveNode};
use crate::client::pathfind::implementations::{Problem, PlayerProblem};
use crate::client::pathfind::incremental::Node;
use crate::client::pathfind::traits::{GoalCheck, Heuristic, Progression, Progressor};
use crate::storage::block::{BlockLocation, BlockState};

struct NoVehicleGoalCheck {
    goal: BlockLocation,
}

impl NoVehicleGoalCheck {
    pub fn new(goal: BlockLocation) -> Self {
        Self {
            goal
        }
    }
}

impl GoalCheck<MoveNode> for NoVehicleGoalCheck {
    fn is_goal(&self, input: &MoveNode) -> bool {
        input.location == self.goal
    }
}


struct NoVehicleHeuristic {
    pub move_cost: f64,
    pub goal: BlockLocation,
}


impl Heuristic<MoveNode> for NoVehicleHeuristic {
    fn heuristic(&self, input: &MoveNode) -> f64 {
        let current = input.location;
        current.dist(self.goal) * self.move_cost
    }
}

pub struct TravelProblem;

impl TravelProblem {
    pub fn new(start: BlockLocation, goal: BlockLocation) -> impl Problem<Node=MoveNode> {
        let heuristic = NoVehicleHeuristic { move_cost: 1.0, goal };
        let start_node = MoveNode::simple(start);
        let goal_checker = NoVehicleGoalCheck::new(goal);
        return PlayerProblem::new(start_node, heuristic, goal_checker, HashMap::new());
    }
}
