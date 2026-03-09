import packageJson from '../../package.json';
import themes from '../../themes.json';
import { history } from '../stores/history';
import { theme } from '../stores/theme';

const commandDescriptions: Record<string, string> = {
	help: 'Display available commands',
	aboutme: 'Display about me info',
	booktime: 'Display booktime info',
	bde: 'Display bde info',
	linkedin: 'Display linkedin info',
	hostname: 'Show current hostname',
	whoami: 'Show current user',
	date: 'Display current date',
	echo: 'Repeat the input text',
	github: 'Open GitHub profile',
	sudo: 'Admin access',
	theme: 'Choose your theme',
	repo: 'Open this repository',
	clear: 'Clears screen',
	email: 'Opens a tab with mailto',
	weather: 'Shows weather info',
	banner: 'Terminal banner',
	exit: 'Displays how to exit',
	blog: 'Open blog'
};

const categories: [string, string[]][] = [
	['General', ['help', 'banner', 'clear', 'exit', 'whoami', 'hostname', 'date']],
	['Navigation', ['github', 'repo', 'blog', 'email', 'linkedin']],
	['Information', ['aboutme', 'weather', 'echo']],
	['Projects', ['booktime', 'bde']],
	['Customization', ['theme', 'sudo']]
];

function openUrl(url: string): void {
	window.open(url);
}

export const commands: Record<string, (args: string[]) => Promise<string> | string> = {
	help: () => {
		let result = 'Available commands:\n\n';

		for (const [category, cmds] of categories) {
			result += `<span style='font-weight: bold; color: #84c138;'>${category}</span>\n`;

			for (const cmd of cmds) {
				const desc = commandDescriptions[cmd] ?? '';
				const padding = ' '.repeat(Math.max(12, cmd.length + 2) - cmd.length);
				result += `<span style='font-weight: bold;'>${cmd}</span>${padding}${desc}\n`;
			}

			result += '\n';
		}

		return result;
	},

	whoami: () => 'guest',

	exit: () => 'Close the tab to exit.',

	hostname: () => window.location.hostname,

	date: () => new Date().toString(),

	booktime: () => {
		openUrl('https://booktime.co');
		return 'Booktime is a mobile application for book reading tracking with book club functionality. Check out <a href="https://booktime.co" target="_blank">https://booktime.co</a> for more info!';
	},

	blog: () => {
		openUrl('https://dev.to/oriiyx');
		return 'Checkout my blog over at <a href="https://dev.to/oriiyx" target="_blank">https://dev.to/oriiyx</a> for more personal thoughts!';
	},

	bde: () => {
		openUrl('https://github.com/oriiyx/bde');
		return 'BDE stands for Boring Database Engine - it takes SQLc ideology (which I love) and tries to implement it in PHP world.';
	},

	linkedin: () => {
		openUrl('https://www.linkedin.com/in/peter-paravinja/');
		return 'Opening <a href="https://www.linkedin.com/in/peter-paravinja" target="_blank">https://www.linkedin.com/in/peter-paravinja</a>';
	},

	github: () => {
		openUrl('https://github.com/oriiyx');
		return 'Opening <a href="https://github.com/oriiyx" target="_blank">https://github.com/oriiyx</a>';
	},

	repo: () => {
		openUrl(packageJson.repository.url);
		return 'Opening repository...';
	},

	email: () => {
		openUrl(`mailto:${packageJson.author.email}`);
		return 'Opening mailto link...';
	},

	sudo: (args: string[]) => {
		if (args.length === 0) return 'Permission denied.';
		return `Permission denied running: ${args[0]}`;
	},

	echo: (args: string[]) => args.join(' '),

	weather: async (args: string[]) => {
		if (args.length === 0) return 'Usage: weather [city]. Example: weather Ljubljana';

		const city = args.join('+');
		const response = await fetch(`https://wttr.in/${city}?ATm`);
		return await response.text();
	},

	theme: (args: string[]) => {
		const usage = `Usage: theme [args].
    [args]:
      ls: list all available themes
      set: set theme to [theme]

    [Examples]:
      theme ls
      theme set gruvboxdark
    `;

		if (args.length === 0) return usage;

		switch (args[0]) {
			case 'ls': {
				const list = themes.map((t) => t.name.toLowerCase()).join(', ');
				return `${list}\nYou can preview all these themes here: ${packageJson.repository.url}/tree/master/docs/themes`;
			}
			case 'set': {
				if (args.length !== 2) return usage;

				const t = themes.find((t) => t.name.toLowerCase() === args[1]);
				if (!t) return `Theme '${args[1]}' not found. Try 'theme ls' to see all available themes.`;

				theme.set(t);
				return `Theme set to ${args[1]}`;
			}
			default:
				return usage;
		}
	},

	clear: () => {
		history.set([]);
		return '';
	},

	banner: () => {
		const version = packageJson.version;
		return `
██████╗ ███████╗████████╗███████╗██████╗     ██████╗
██╔══██╗██╔════╝╚══██╔══╝██╔════╝██╔══██╗    ██╔══██╗
██████╔╝█████╗     ██║   █████╗  ██████╔╝    ██████╔╝
██╔═══╝ ██╔══╝     ██║   ██╔══╝  ██╔══██╗    ██╔═══╝
██║     ███████╗   ██║   ███████╗██║  ██║    ██║██╗
╚═╝     ╚══════╝   ╚═╝   ╚══════╝╚═╝  ╚═╝    ╚═╝╚═╝  v${version}

Peter Paravinja | Full-Stack Developer | Golang | TypeScript | PHP | Rust | Python
Working on web applications, mobile applications and open-source tools.
Type 'help' to see list of available commands.
`;
	},

	aboutme: () => `
<span style='font-weight: bold; color: #84c138;'>Hey there! I'm Peter Paravinja</span>, a full-stack developer with 6 years of experience in building
complex web applications and a bit of mobile development.

<span style='font-weight: bold;'>CURRENT WORK</span>:
- Working at Netis on cryptographic applications, passkey technology while tackling infrastructure problems
- Developing SQLC port for PHP and MySQL (rewritten in Rust)
- Creating interactive CLI-based tools and applications

<span style='font-weight: bold;'>SKILLS</span>:
- Languages: Golang, JavaScript/TypeScript, Rust, PHP, Python
- Full development cycle: from design to deployment and marketing
- Solo developed Booktime.co, a book reading tracking app with book club functionality

<span style='font-weight: bold;'>PROJECTS</span>:
- Booktime.co: A social book-tracking platform with book club features
  (React Native, Remix, Golang, infrastructure, design, marketing)
- Use 'booktime' command for more info!
- Use 'bde' command for more info!

<span style='font-weight: bold;'>CONNECT</span>:
- Blog: <a href="https://oriiyx.dev/" target="_blank">https://dev.to/oriiyx/</a>
- LinkedIn: <a href="https://www.linkedin.com/in/peter-paravinja/" target="_blank">https://www.linkedin.com/in/peter-paravinja/</a>
- GitHub: <a href="https://github.com/oriiyx" target="_blank">https://github.com/oriiyx</a>

I'm always looking for new challenges and opportunities to improve my skills.
Feel free to reach out via LinkedIn or GitHub if you'd like to connect!
`
};