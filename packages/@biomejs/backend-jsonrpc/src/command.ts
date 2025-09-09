/**
 * Gets the path of the Check binary for the current platform
 *
 * @returns Filesystem path to the binary, or null if no prebuilt distribution exists for the current platform
 */
export function getCommand(): string | null {
	const { platform, arch } = process;

	type PlatformPaths = {
		[P in NodeJS.Platform]?: {
			[A in NodeJS.Architecture]?: string;
		};
	};

	const PLATFORMS: PlatformPaths = {
		win32: {
			x64: "@checkjs/cli-win32-x64/check.exe",
			arm64: "@checkjs/cli-win32-arm64/check.exe",
		},
		darwin: {
			x64: "@checkjs/cli-darwin-x64/check",
			arm64: "@checkjs/cli-darwin-arm64/check",
		},
		linux: {
			x64: "@checkjs/cli-linux-x64/check",
			arm64: "@checkjs/cli-linux-arm64/check",
		},
	};

	const binPath = PLATFORMS?.[platform]?.[arch];
	if (!binPath) {
		return null;
	}

	return require.resolve(binPath);
}
