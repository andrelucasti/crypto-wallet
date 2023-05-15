use uuid::Uuid;
use crate::portfolio::portfolio::Portfolio;
use crate::portfolio::portfolio;
use crate::portfolio::portfolio_repository::{PortfolioRepository};

pub fn execute(portfolio: Portfolio, repository: &mut PortfolioRepository) {
    repository.save(portfolio)
}


#[test]
fn should_save_portfolio(){
    let portfolio_name = String::from("game_token");
    let user_id = Uuid::new_v4();
    let portfolio = Portfolio::new(portfolio_name.clone(), user_id.clone());

    let mut repository = PortfolioRepository::new();
    execute(portfolio, &mut repository);


    let portfolios = repository.find_all();
    if let Some(value) = portfolios.get(0) {
        assert_eq!(portfolio_name, value.name);
        assert_eq!(user_id, value.user_id);
    } else {
        Err(String::from("Got an error at save a new portfolio"))
    }
}