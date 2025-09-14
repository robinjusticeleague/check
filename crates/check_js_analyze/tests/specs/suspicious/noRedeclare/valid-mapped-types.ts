/* should not generate diagnostics */
// See https://github.com/checkjs/check/issues/953
type X = never;
type Y = never;
export const MyMappingPbToGql: {
    [key in X]: never;
} = {};
export const MyOtherMappingPbToGql: {
    [key in Y]: never;
} = {};
