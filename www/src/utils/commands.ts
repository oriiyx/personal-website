import { CommandRegistry } from 'peter-paravinja-personal-website';
import packageJson from '../../package.json';
import themes from '../../themes.json';
import { history } from '../stores/history';
import { theme } from '../stores/theme';

const commandRegistry = new CommandRegistry();

export const commands: Record<string, (args: string[]) => Promise<string> | string> = {
	help: () => commandRegistry.help(),
	hostname: () => commandRegistry.hostname(),
	whoami: () => commandRegistry.whoami(),
	aboutme: () => commandRegistry.aboutme(),
	booktime: () => commandRegistry.booktime(),
	bde: () => commandRegistry.bde(),
	blog: () => commandRegistry.blog(),
	linkedin: () => commandRegistry.linkedin(),
	date: () => commandRegistry.date(),
	echo: (args: string[]) => commandRegistry.echo(args),
	github: () => commandRegistry.github(),
	sudo: (args: string[]) => commandRegistry.sudo(args),
	theme: (args: string[]) => {
		const usage = `Usage: theme [args].
    [args]:
      ls: list all available themes
      set: set theme to [theme]

    [Examples]:
      theme ls
      theme set gruvboxdark
    `;
		if (args.length === 0) {
			return usage;
		}

		switch (args[0]) {
			case 'ls': {
				let result = themes.map((t) => t.name.toLowerCase()).join(', ');
				result += `You can preview all these themes here: ${packageJson.repository.url}/tree/master/docs/themes`;

				return result;
			}

			case 'set': {
				if (args.length !== 2) {
					return usage;
				}

				const selectedTheme = args[1];
				const t = themes.find((t) => t.name.toLowerCase() === selectedTheme);

				if (!t) {
					return `Theme '${selectedTheme}' not found. Try 'theme ls' to see all available themes.`;
				}

				theme.set(t);

				return `Theme set to ${selectedTheme}`;
			}

			default: {
				return usage;
			}
		}
	},
	repo: () => commandRegistry.repo(packageJson.repository.url),
	clear: () => {
		history.set([]);

		return '';
	},
	email: () => commandRegistry.email(packageJson.author.email),
	weather: async (args: string[]) => commandRegistry.weather(args),
	banner: () => commandRegistry.banner(packageJson.version),
	exit: () => commandRegistry.exit()
};
