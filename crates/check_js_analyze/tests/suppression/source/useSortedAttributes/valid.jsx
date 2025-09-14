/** should not generate diagnostics */
<>
	{/* check-ignore assist/source/useSortedAttributes: test */}
	<Hello
		lastName="Smith"
		firstName="John"
	/>;
	{/* check-ignore assist/source/useSortedAttributes: test */}
	<Hello tel={5555555} address="NY" {...this.props} lastName="Smith" firstName="John" />;
	{/* check-ignore assist/source/useSortedAttributes: test */}
	<Hello a10="" a9="" A="" ></Hello>;
</>
