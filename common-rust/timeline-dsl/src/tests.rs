use common_lib::*;

use crate::parser::*;

fn assert_display(op: &TimeLineOp, expected: &str) {
    assert_eq!(format!("{}", op), expected);
}

#[test]
fn latest_event_to_state() {
    let result = parse(r#"latest_event_to_state("status")"#).unwrap();
    assert_display(&result.op, "TlLatestEventToState(status)");
    assert!(result.aggregation.is_none());
}

#[test]
fn has_existed_with_equals() {
    let result = parse(r#"has_existed(status == "active")"#).unwrap();
    assert_display(&result.op, "TlHasExisted(status == active)");
}

#[test]
fn has_existed_with_greater_than() {
    let result = parse("has_existed(score > 100)").unwrap();
    assert_display(&result.op, "TlHasExisted(score > 100)");
}

#[test]
fn has_existed_with_less_than() {
    let result = parse("has_existed(health < 50)").unwrap();
    assert_display(&result.op, "TlHasExisted(health < 50)");
}

#[test]
fn has_existed_within() {
    let result = parse(r#"has_existed_within(status == "error", 3600)"#).unwrap();
    assert_display(&result.op, "TlHasExistedWithin(status == error, 3600)");
}

#[test]
fn equal_to_string() {
    let result = parse(r#"latest_event_to_state("status") == "active""#).unwrap();
    assert_display(&result.op, "EqualTo(TlLatestEventToState(status), active)");
}

#[test]
fn greater_than_int() {
    let result = parse(r#"latest_event_to_state("score") > 100"#).unwrap();
    assert_display(&result.op, "GreaterThan(TlLatestEventToState(score), 100)");
}

#[test]
fn greater_than_or_equal() {
    let result = parse(r#"latest_event_to_state("score") >= 50"#).unwrap();
    assert_display(
        &result.op,
        "GreaterThanOrEqual(TlLatestEventToState(score), 50)",
    );
}

#[test]
fn less_than() {
    let result = parse(r#"latest_event_to_state("health") < 20"#).unwrap();
    assert_display(&result.op, "LessThan(TlLatestEventToState(health), 20)");
}

#[test]
fn less_than_or_equal() {
    let result = parse(r#"latest_event_to_state("health") <= 0"#).unwrap();
    assert_display(
        &result.op,
        "LessThanOrEqual(TlLatestEventToState(health), 0)",
    );
}

#[test]
fn compare_float() {
    let result = parse(r#"latest_event_to_state("temperature") > 36.5"#).unwrap();
    assert_display(
        &result.op,
        "GreaterThan(TlLatestEventToState(temperature), 36.5)",
    );
}

#[test]
fn compare_bool() {
    let result = parse(r#"latest_event_to_state("flag") == true"#).unwrap();
    assert_display(&result.op, "EqualTo(TlLatestEventToState(flag), true)");
}

#[test]
fn negation() {
    let result = parse(r#"!has_existed(error == "fatal")"#).unwrap();
    assert_display(&result.op, "Not(TlHasExisted(error == fatal))");
}

#[test]
fn and_two_timelines() {
    let result = parse(
        r#"has_existed(status == "active") && has_existed(region == "us")"#,
    )
    .unwrap();
    assert_display(
        &result.op,
        "And(TlHasExisted(status == active), TlHasExisted(region == us))",
    );
}

#[test]
fn or_two_timelines() {
    let result = parse(
        r#"has_existed(status == "a") || has_existed(status == "b")"#,
    )
    .unwrap();
    assert_display(
        &result.op,
        "Or(TlHasExisted(status == a), TlHasExisted(status == b))",
    );
}

#[test]
fn and_binds_tighter_than_or() {
    // a || b && c  should parse as  a || (b && c)
    let result = parse(
        r#"has_existed(x == 1) || has_existed(y == 2) && has_existed(z == 3)"#,
    )
    .unwrap();
    assert_display(
        &result.op,
        "Or(TlHasExisted(x == 1), And(TlHasExisted(y == 2), TlHasExisted(z == 3)))",
    );
}

#[test]
fn parentheses_override_precedence() {
    let result = parse(
        r#"(has_existed(x == 1) || has_existed(y == 2)) && has_existed(z == 3)"#,
    )
    .unwrap();
    assert_display(
        &result.op,
        "And(Or(TlHasExisted(x == 1), TlHasExisted(y == 2)), TlHasExisted(z == 3))",
    );
}

#[test]
fn duration_where() {
    let result = parse(r#"duration_where(has_existed(online == true))"#).unwrap();
    assert_display(
        &result.op,
        "TlDurationWhere(TlHasExisted(online == true))",
    );
}

#[test]
fn duration_in_cur_state() {
    let result = parse(r#"duration_in_cur_state(latest_event_to_state("status") == "idle")"#).unwrap();
    assert_display(
        &result.op,
        "TlDurationInCurState(EqualTo(TlLatestEventToState(status), idle))",
    );
}

#[test]
fn complex_nested() {
    let result = parse(
        r#"duration_where(has_existed(status == "active") && !has_existed_within(error > 0, 300))"#,
    )
    .unwrap();
    assert_display(
        &result.op,
        "TlDurationWhere(And(TlHasExisted(status == active), Not(TlHasExistedWithin(error > 0, 300))))",
    );
}

#[test]
fn aggregate_single_function() {
    let result = parse(
        r#"has_existed(status == "active") | aggregate(group_by="region", count)"#,
    )
    .unwrap();
    assert_display(&result.op, "TlHasExisted(status == active)");
    let agg = result.aggregation.unwrap();
    assert_eq!(agg.group_by, "region");
    assert_eq!(agg.functions, vec![AggregationFunction::Count]);
}

#[test]
fn aggregate_multiple_functions() {
    let result = parse(
        r#"latest_event_to_state("score") > 0 | aggregate(group_by="team", count, sum, avg, min, max)"#,
    )
    .unwrap();
    let agg = result.aggregation.unwrap();
    assert_eq!(agg.group_by, "team");
    assert_eq!(
        agg.functions,
        vec![
            AggregationFunction::Count,
            AggregationFunction::Sum,
            AggregationFunction::Avg,
            AggregationFunction::Min,
            AggregationFunction::Max,
        ]
    );
}

#[test]
fn full_complex_query_with_aggregation() {
    let input = r#"
        duration_where(
            has_existed(status == "active")
            && !(has_existed_within(error > 0, 300))
        ) | aggregate(group_by="region", count, avg)
    "#;
    let result = parse(input).unwrap();
    assert_display(
        &result.op,
        "TlDurationWhere(And(TlHasExisted(status == active), Not(TlHasExistedWithin(error > 0, 300))))",
    );
    let agg = result.aggregation.unwrap();
    assert_eq!(agg.group_by, "region");
    assert_eq!(
        agg.functions,
        vec![AggregationFunction::Count, AggregationFunction::Avg]
    );
}

#[test]
fn chained_and() {
    let result = parse(
        "has_existed(a == 1) && has_existed(b == 2) && has_existed(c == 3)",
    )
    .unwrap();
    // Left-associative: And(And(a, b), c)
    assert_display(
        &result.op,
        "And(And(TlHasExisted(a == 1), TlHasExisted(b == 2)), TlHasExisted(c == 3))",
    );
}

#[test]
fn chained_or() {
    let result = parse(
        "has_existed(a == 1) || has_existed(b == 2) || has_existed(c == 3)",
    )
    .unwrap();
    assert_display(
        &result.op,
        "Or(Or(TlHasExisted(a == 1), TlHasExisted(b == 2)), TlHasExisted(c == 3))",
    );
}

#[test]
fn double_negation() {
    let result = parse("!!has_existed(x == 1)").unwrap();
    assert_display(&result.op, "Not(Not(TlHasExisted(x == 1)))");
}

#[test]
fn error_unterminated_string() {
    let err = parse(r#"latest_event_to_state("oops)"#).unwrap_err();
    assert!(err.message.contains("unterminated string"));
}

#[test]
fn error_missing_rparen() {
    let err = parse(r#"latest_event_to_state("x""#).unwrap_err();
    assert!(err.message.contains("expected ')'"));
}

#[test]
fn error_unexpected_token() {
    let err = parse("42").unwrap_err();
    assert!(err.message.contains("unexpected token"));
}

#[test]
fn error_bad_predicate_op() {
    let err = parse("has_existed(col >= 1)").unwrap_err();
    assert!(err.message.contains("predicate operator"));
}

#[test]
fn error_missing_aggregate_function() {
    let err =
        parse(r#"has_existed(x == 1) | aggregate(group_by="r", )"#).unwrap_err();
    assert!(err.message.contains("aggregation function"));
}

#[test]
fn cirr_query() {
    let input = r#"
        duration_where(
            has_existed(playerStateChange == "play")
            && !has_existed_within(playerStateChange == "seek", 5)
            && latest_event_to_state("playerStateChange") == "buffer"
        )
    "#;
    let result = parse(input).unwrap();
    assert_display(
        &result.op,
        "TlDurationWhere(And(And(TlHasExisted(playerStateChange == play), Not(TlHasExistedWithin(playerStateChange == seek, 5))), EqualTo(TlLatestEventToState(playerStateChange), buffer)))",
    );
}

#[test]
fn cirr_query_with_aggregation() {
    let input = r#"
        duration_where(
            has_existed(playerStateChange == "play")
            && !has_existed_within(playerStateChange == "seek", 5)
            && latest_event_to_state("playerStateChange") == "buffer"
        ) | aggregate(group_by="cdn-x", count, sum, avg)
    "#;
    let result = parse(input).unwrap();
    assert_display(
        &result.op,
        "TlDurationWhere(And(And(TlHasExisted(playerStateChange == play), Not(TlHasExistedWithin(playerStateChange == seek, 5))), EqualTo(TlLatestEventToState(playerStateChange), buffer)))",
    );
    let agg = result.aggregation.unwrap();
    assert_eq!(agg.group_by, "cdn-x");
    assert_eq!(
        agg.functions,
        vec![AggregationFunction::Count, AggregationFunction::Sum, AggregationFunction::Avg],
    );
}

#[test]
fn whitespace_insensitive() {
    let compact = parse(r#"has_existed(x==1)&&has_existed(y==2)"#).unwrap();
    let spaced = parse(r#"has_existed(x == 1) && has_existed(y == 2)"#).unwrap();
    assert_eq!(format!("{}", compact.op), format!("{}", spaced.op));
}
