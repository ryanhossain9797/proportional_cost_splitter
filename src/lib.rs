/// Calculates the overall amount for each person's costs.
///
/// # Arguments
///
/// * `costs` - A vector of f64 values representing individual costs before VAT, Service Charge etc.
/// * `total` - The total cost with VAT, Service Charge etc.
///
/// # Returns
///
/// A vector of f64 values representing the the final amount for each person.
///
/// # Example
///
/// ```
/// # use  proportional_cost_splitter_lib::*;
/// let total = 120.0;
/// let costs = 
///     vec![
///         (Some("Raiyan".to_string()), 10.0),
///         (Some("Nur".to_string()), 30.0),
///         (Some("Hasan".to_string()), 20.0),
///     ];
/// 
/// let scaled_amounts = 
///     scale_to_total(costs, total)
///         .into_iter()
///         .map(|(_, cost)| cost) //Ignore name
///         .collect::<Vec<_>>();
/// 
/// assert_eq!(
///     scaled_amounts,
///     vec![
///         20.0,
///         60.0,
///         40.0,
///     ]);
/// ```
pub fn scale_to_total (
    initial_costs:  Vec<(Option<String>, f64)>, 
    final_cost: f64) -> Vec<(String, f64)> {
    let sum: f64 = initial_costs.iter().map(|(_, cost)| cost).sum();

    let ratio = final_cost / sum;

    let scaled_costs = 
        initial_costs
            .into_iter()
            .map(|(maybe_name, cost)| (maybe_name.unwrap_or_else(||format!("{cost}")), cost * ratio))
            .collect::<Vec<_>>();

    scaled_costs
}