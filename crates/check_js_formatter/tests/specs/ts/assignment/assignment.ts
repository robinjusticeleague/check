//break after operator layout
loooooooooooooooooooooooooong1 = "looooooooooooooooooooooooooooooooooooooooooog"!;
loooooooooooooooooooooooooong2 = void void "looooooooooooooooooooooooooooooooooooooooooog"!;
// check-ignore format: test
  loooooooooooooooooooooooooong6    =
	void    "looooooooooooooooooooooooooooooooooooooooooog"!;
loooooooooooooooooooooooooong7    =
	// check-ignore format: test
	!      "looooooooooooooooooooooooooooooooooooooooooog"!;

const gitBaseExtension = extensions.getExtension<GitBaseExtension>(
	"vscode.git-base",
)!.exports;
