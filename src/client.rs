use std::collections::HashMap;

use calculator::calculator_client::CalculatorClient;
use calculator::{CalculationData, Product, Promotion, Tag, Qualification, Exclusions};

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalculatorClient::connect("http://[::1]:50051").await?;

    let mut products = HashMap::new();
    let mut tags = HashMap::new();
    let mut qualifications = HashMap::new();
    let mut promotions = HashMap::new();
    let mut exclusions = HashMap::new();

    tags.insert(
        1,
         Tag {
            id: 1
        }
    );

    products.insert(
        1,
        Product {
            id: 1,
            price: 2,
            tags: tags.clone()
        }
    );


    qualifications.insert(
        1,
        Qualification {
            tags: tags,
            min: 1,
            max: 2,
        }
    );

    exclusions.insert(
        1, 
        Exclusions {
            products: HashMap::new(),
            promotions: HashMap::new()
        }
    );

    promotions.insert(
        1,
        Promotion {
            id: 1,
            qualifications: qualifications,
            exclusions: exclusions,
            rule: "new_price".to_string(),
            price: 4,
        }
    );

    let request = tonic::Request::new(
        CalculationData {
            products: products,
            promotions: promotions
        }
    );

    let response = client.run(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}