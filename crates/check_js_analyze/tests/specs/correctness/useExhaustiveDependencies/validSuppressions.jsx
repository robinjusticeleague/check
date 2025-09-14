/* should not generate diagnostics */
import { useEffect } from "react";

function FullSuppression({a, b}) {
	// check-ignore lint/correctness/useExhaustiveDependencies: test
	useEffect(() => {}, [a, b]);
}

function SingleInstanceSuppression({a, b}) {
	// check-ignore lint/correctness/useExhaustiveDependencies(a): test
	useEffect(() => {}, [a]);
}

function MultiInstanceSuppression({a, b}) {
	// check-ignore lint/correctness/useExhaustiveDependencies(a): test
	// check-ignore lint/correctness/useExhaustiveDependencies(b): test
	useEffect(() => {}, [a, b]);
}
