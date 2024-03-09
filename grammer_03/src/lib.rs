
const PI: f64 = 3.1415926;
static  V: Vec<u8> = Vec::new();

#[allow(dead_code)]
fn where_is_pi() {
    let r = 10f64;
    println!(
        "addr(r):{:p}, addr(PI):{:p}, addr(V):{:p} area is:{}",
        &r,
        &PI,
        &V,
        PI * r *r
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_where_is_pi() {
        where_is_pi();
    }
}