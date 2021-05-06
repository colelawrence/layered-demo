use layered_amount::Amount;
use layered_nlp::*;

struct MoneyResolver;

#[derive(Debug)]
enum Currency {
    USD,
    Pound,
}

#[derive(Debug)]
struct Money(Currency, Amount);

impl Resolver for MoneyResolver {
    type Attr = Money;
    fn go(
        &self,
        selection: layered_nlp::LLSelection,
    ) -> Vec<layered_nlp::LLCursorAssignment<Self::Attr>> {
        selection
            .find_by(&x::seq((
                // x::any_of((x::attr_eq(&'$'), x::attr_eq(&'£'))),
                x::token_has_any(&['$', '£']),
                x::attr::<Amount>(),
            )))
            .into_iter()
            .map(|(selection, (which, amount))| {
                selection.finish_with_attr(Money(
                    match which {
                        '$' => Currency::USD,
                        '£' => Currency::Pound,
                        _ => unreachable!(),
                    },
                    amount.clone(),
                ))
            })
            .collect()
    }
}

#[cfg(test)]
mod tests;
