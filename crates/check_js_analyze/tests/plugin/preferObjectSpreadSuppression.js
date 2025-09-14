// check-ignore lint/plugin/preferObjectSpreadSuppression: reason
Object.assign({ foo: 'bar'}, baz);

// check-ignore-start lint/plugin/preferObjectSpreadSuppression: reason
Object.assign({}, {foo: 'bar'});
// check-ignore-end lint/plugin/preferObjectSpreadSuppression: reason

// if no name is specified, should suppress all plugins
// check-ignore lint/plugin: reason
Object.assign({}, foo);

// only suppress specified plugin
// check-ignore lint/plugin/anotherPlugin: reason
Object.assign({ foo: 'bar'}, baz);
