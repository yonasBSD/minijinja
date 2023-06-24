#![cfg(all(feature = "datetime", feature = "timezone"))]
use minijinja::context;
use similar_asserts::assert_eq;

#[test]
fn test_datetimeformat() {
    let mut env = minijinja::Environment::new();
    env.add_global("TIMEZONE", "Europe/Vienna");
    env.add_global("DATETIME_FORMAT", "[hour]:[minute]");
    minijinja_contrib::add_to_environment(&mut env);

    let expr = env
        .compile_expression("1687624642.5|datetimeformat(format=format)")
        .unwrap();

    assert_eq!(
        expr.eval(context!(format => "short")).unwrap().to_string(),
        "2023-06-24 18:37"
    );
    assert_eq!(
        expr.eval(context!(format => "medium")).unwrap().to_string(),
        "Jun 24 2023 18:37"
    );
    assert_eq!(
        expr.eval(context!(format => "long")).unwrap().to_string(),
        "June 24 2023 18:37:22"
    );
    assert_eq!(
        expr.eval(context!(format => "full")).unwrap().to_string(),
        "Saturday, June 24 2023 18:37:22.5"
    );
    assert_eq!(
        expr.eval(context!(format => "unix")).unwrap().to_string(),
        "1687624642"
    );
    assert_eq!(
        expr.eval(context!(format => "iso")).unwrap().to_string(),
        "2023-06-24T18:37:22+02:00"
    );

    let expr = env
        .compile_expression("1687624642|datetimeformat(tz='Europe/Moscow')")
        .unwrap();
    assert_eq!(expr.eval(()).unwrap().to_string(), "19:37");
}

#[test]
fn test_dateformat() {
    let mut env = minijinja::Environment::new();
    env.add_global("TIMEZONE", "Europe/Vienna");
    env.add_global("DATE_FORMAT", "[year]-[month]");
    minijinja_contrib::add_to_environment(&mut env);

    let expr = env
        .compile_expression("1687624642.5|dateformat(format=format)")
        .unwrap();

    assert_eq!(
        expr.eval(context!(format => "short")).unwrap().to_string(),
        "2023-06-24"
    );
    assert_eq!(
        expr.eval(context!(format => "medium")).unwrap().to_string(),
        "Jun 24 2023"
    );
    assert_eq!(
        expr.eval(context!(format => "long")).unwrap().to_string(),
        "June 24 2023"
    );
    assert_eq!(
        expr.eval(context!(format => "full")).unwrap().to_string(),
        "Saturday, June 24 2023"
    );

    let expr = env
        .compile_expression("1687624642|dateformat(tz='Europe/Moscow')")
        .unwrap();
    assert_eq!(expr.eval(()).unwrap().to_string(), "2023-06");
}

#[test]
fn test_timeformat() {
    let mut env = minijinja::Environment::new();
    env.add_global("TIMEZONE", "Europe/Vienna");
    env.add_global("TIME_FORMAT", "[hour]:[minute]");
    minijinja_contrib::add_to_environment(&mut env);

    let expr = env
        .compile_expression("1687624642.5|timeformat(format=format)")
        .unwrap();

    assert_eq!(
        expr.eval(context!(format => "short")).unwrap().to_string(),
        "18:37"
    );
    assert_eq!(
        expr.eval(context!(format => "medium")).unwrap().to_string(),
        "18:37"
    );
    assert_eq!(
        expr.eval(context!(format => "long")).unwrap().to_string(),
        "18:37:22"
    );
    assert_eq!(
        expr.eval(context!(format => "full")).unwrap().to_string(),
        "18:37:22.5"
    );
    assert_eq!(
        expr.eval(context!(format => "unix")).unwrap().to_string(),
        "1687624642"
    );
    assert_eq!(
        expr.eval(context!(format => "iso")).unwrap().to_string(),
        "2023-06-24T18:37:22+02:00"
    );

    let expr = env
        .compile_expression("1687624642|timeformat(tz='Europe/Moscow')")
        .unwrap();
    assert_eq!(expr.eval(()).unwrap().to_string(), "19:37");
}