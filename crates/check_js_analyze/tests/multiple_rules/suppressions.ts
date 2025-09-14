///! lint/suspicious/noDoubleEquals
///! lint/suspicious/noConsole
///! lint/style/useArrayLiterals

/* check-ignore lint: ... */
console.log(1 == 0)

/* check-ignore lint/suspicious/noConsole: ... */
console.log(1 == 0)

/* check-ignore lint/suspicious/noConsole: ... */
/* check-ignore lint/suspicious/noDoubleEquals: ... */
console.log(1 == 0)

/* check-ignore lint/suspicious/noConsole: ... */
/* check-ignore lint/suspicious/noDoubleEquals: ... */
console.log(1 == Array())

/* check-ignore lint/suspicious/noConsole: ... */ /* check-ignore lint/suspicious/noDoubleEquals: ... */
console.log(1 == 0)
/* check-ignore lint/suspicious/noConsole: ... */ /* check-ignore lint/suspicious/noDoubleEquals: ... */
console.log(1 == Array())

/* check-ignore lint/suspicious/noConsole: ... */ /* check-ignore lint/suspicious/noDoubleEquals: ... */ console.log(1 == 0)

/* check-ignore lint/suspicious/noConsole: ... */ /* check-ignore lint/suspicious/noDoubleEquals: ... */ console.log(1 == Array())

if (1) {
    /* check-ignore lint/suspicious/noConsole: ... */
    console.log(1 == 0)
    /* check-ignore lint/suspicious/noDoubleEquals: ... */
}

if (1) {
    /* check-ignore lint/suspicious/noConsole: ... */
    console.log(1 == Array())
    /* check-ignore lint/suspicious/noDoubleEquals: ... */
}
