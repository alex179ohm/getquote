use chrono::Utc;
use futures_executor::block_on;
use std::fmt::Display;
use yahoo_finance_api as yahoo;

#[derive(Debug)]
struct Stock {
    name: String,
    currency: Currency,
}

impl Stock {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn currency(&self) -> Currency {
        self.currency.clone()
    }
}

impl From<String> for Stock {
    fn from(name: String) -> Self {
        let currency = if name.split(".").collect::<Vec<&str>>().len() > 1 {
            Currency::EUR
        } else {
            Currency::USD
        };
        Stock { name, currency }
    }
}

#[derive(Debug, Clone)]
enum Currency {
    EUR,
    USD,
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::EUR => write!(f, "€"),
            Currency::USD => write!(f, "$"),
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    tracing::debug!("args len: {}", args.len());

    if args.len() > 1 {
        let stock: Stock = args[1].clone().into();
        tracing::debug!("stock name: {:?}", stock.name());
        tracing::debug!("stock currency {:?}", stock.currency());

        let provider = yahoo::YahooConnector::new();

        let resp = match block_on(provider.get_latest_quotes(&args[1], "1d")) {
            Err(e) => panic!("Error on retrieving stocke quote: {:?}: {}", stock, e),
            Ok(r) => r,
        };

        let quote = resp.last_quote().unwrap();

        let dt = Utc::now();

        println!(
            "{} {} {}{:.02}",
            dt.format("%Y/%m/%d %H:%M:%S"),
            stock.name(),
            stock.currency(),
            quote.close
        );
    }
}
