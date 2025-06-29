use crate::*;

pub fn tool_err_msg(outcome: &Result<ExecutionFinalResult>) -> String {
    match outcome {
        Ok(res) => {
            let mut msg = "".to_string();
            for r in res.receipt_failures() {
                match r.clone().into_result() {
                    Ok(_) => {}
                    Err(err) => {
                        msg += &format!("{:?}", err);
                        msg += "\n";
                    }
                }
            }
            msg
        }
        Err(err) => err.to_string(),
    }
}

#[macro_export]
macro_rules! check {
    ($exec_func: expr) => {
        let outcome = $exec_func.await.unwrap();
        assert!(outcome.is_success() && outcome.receipt_failures().is_empty());
    };
    (logs $exec_func: expr) => {
        let outcome = $exec_func.await.unwrap();
        assert!(outcome.is_success() && outcome.receipt_failures().is_empty());
        println!("logs: {:#?}", outcome.logs());
    };
    (print $exec_func: expr) => {
        let outcome = $exec_func.await;
        let err_msg = tool_err_msg(&outcome);
        println!("==>");
        if err_msg.is_empty() {
            let o = outcome.unwrap();
            println!("logs: {:#?}", o.logs());
        } else {
            println!("errors: {}", err_msg);
        }
        println!("<==");
    };
    (print $prefix: literal $exec_func: expr) => {
        let outcome = $exec_func.await;
        let err_msg = tool_err_msg(&outcome);
        println!("==>");
        if err_msg.is_empty() {
            let o = outcome.unwrap();
            println!("{} logs: {:#?}", $prefix, o.logs());
        } else {
            println!("{} errors: {}", $prefix, err_msg);
        }
        println!("<==");
    };
    (printr $exec_func: expr) => {
        let outcome = $exec_func.await;
        let err_msg = tool_err_msg(&outcome);
        println!("==>");
        if err_msg.is_empty() {
            let o = outcome.unwrap();
            println!("logs: {:#?}", o.logs());
            println!("");
            println!("return: {:#?}", o.json::<near_sdk::serde_json::Value>());
        } else {
            println!("errors: {}", err_msg);
        }
        println!("<==");
    };
    (printr $prefix: literal $exec_func: expr) => {
        let outcome = $exec_func.await;
        let err_msg = tool_err_msg(&outcome);
        println!("==>");
        if err_msg.is_empty() {
            let o = outcome.unwrap();
            println!("{} logs: {:#?}", $prefix, o.logs());
            println!("");
            println!(
                "{} return: {:#?}",
                $prefix,
                o.json::<near_sdk::serde_json::Value>()
            );
        } else {
            println!("{} errors: {}", $prefix, err_msg);
        }
        println!("<==");
    };
    (view $exec_func: expr) => {
        let query_result = $exec_func.await.unwrap();
        println!("{:?}", query_result);
    };
    (view $prefix: literal $exec_func: expr) => {
        let query_result = $exec_func.await.unwrap();
        println!("{} {:#?}", $prefix, query_result);
    };
    ($exec_func: expr, $err_info: expr) => {
        let err_msg = tool_err_msg(&$exec_func.await);
        if !err_msg.contains($err_info) {
            assert!(false, "panic msg: {}", err_msg);
        }
    };
    (logs $exec_func: expr, $err_info: expr) => {
        let outcome = $exec_func.await;
        assert!(tool_err_msg(&outcome).contains($err_info));
        println!("failed logs: {:#?}", outcome.unwrap().logs());
    };
}
