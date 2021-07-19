use ethnographers::proposed_dates;

fn main() {
    let fact_form1 = vec![(1, 2), (3, 4), (4, 6)];
    let fact_form2 = vec![(2, 3), (5, 6)];
    let n = 6;

    if let Some((proposed_births, proposed_deaths)) = proposed_dates(&fact_form1, &fact_form2, n) {
        for i in 1..=n as usize {
            println!(
                "birth {} = {}, death {} = {}",
                i,
                proposed_births[i - 1],
                i,
                proposed_deaths[i - 1]
            );
        }
    } else {
        println!("inconsistent dates!");
    }
}
