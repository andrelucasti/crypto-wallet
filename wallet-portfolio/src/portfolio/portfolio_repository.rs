use crate::portfolio::portfolio::Portfolio;

pub struct PortfolioRepository {
    table: Vec<Portfolio>
}

impl PortfolioRepository {
    pub fn new() -> Self {
        PortfolioRepository {
            table: vec![],
        }
    }

    pub fn save(&mut self, portfolio: Portfolio){
        self.table.push(portfolio)
    }

    pub fn find_all(self) -> Vec<Portfolio>{
        self.table
    }
}