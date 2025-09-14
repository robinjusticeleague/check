import { useEffect } from "react";

function SingleInstanceSuppressionWrong({a, b}) {
	// check-ignore lint/correctness/useExhaustiveDependencies(b): test
	useEffect(() => {}, [a]);
}

function SingleInstanceSuppressionDuplicate({a, b}) {
	// check-ignore lint/correctness/useExhaustiveDependencies(b): test
	// check-ignore lint/correctness/useExhaustiveDependencies(b): test
	useEffect(() => {}, [b]);
}

function SingleInstanceSuppressionNotEnough({a, b}) {
	// check-ignore lint/correctness/useExhaustiveDependencies(a): test
	useEffect(() => {}, [a, b]);
}

function SingleInstanceSuppressionNotEnough2({a, b}) {
	// check-ignore lint/correctness/useExhaustiveDependencies(b): test
	useEffect(() => {}, [a, b]);
}

function MultiInstanceSuppressionSomeWrong({a, b, c}) {
	// check-ignore lint/correctness/useExhaustiveDependencies(a): test
	// check-ignore lint/correctness/useExhaustiveDependencies(c): test
	useEffect(() => {}, [a, b]);
}

function MultiInstanceSuppressionAllWrong({a, b, c, d}) {
	// check-ignore lint/correctness/useExhaustiveDependencies(c): test
	// check-ignore lint/correctness/useExhaustiveDependencies(d): test
	useEffect(() => {}, [a, b]);
}
