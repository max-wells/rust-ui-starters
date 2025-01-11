/** @type {import('tailwindcss').Config} */
module.exports = {
	darkMode: "class",
	content: {
		files: ["*.html", "./src/**/*.rs", "./input.css"],
	},
	theme: {
		container: {
			center: true,
			padding: "2rem",
			screens: {
				"2xl": "1400px",
			},
		},
		extend: {
			fontFamily: {
				opensans: ["Open Sans", "sans-serif"],
				robotomono: ["Roboto Mono", "monospace"],
			},
			colors: {
				border: "hsl(var(--border))",
				input: "hsl(var(--input))",
				ring: "hsl(var(--ring))",
				background: "hsl(var(--background))",
				foreground: "hsl(var(--foreground))",
				primary: {
					DEFAULT: "hsl(var(--primary))",
					foreground: "hsl(var(--primary-foreground))",
				},
				secondary: {
					DEFAULT: "hsl(var(--secondary))",
					foreground: "hsl(var(--secondary-foreground))",
				},
				success: {
					DEFAULT: "hsl(var(--success))",
					foreground: "hsl(var(--success-foreground))",
				},
				info: {
					DEFAULT: "hsl(var(--info))",
					foreground: "hsl(var(--info-foreground))",
				},
				warning: {
					DEFAULT: "hsl(var(--warning))",
					foreground: "hsl(var(--warning-foreground))",
				},
				error: {
					DEFAULT: "hsl(var(--error))",
					foreground: "hsl(var(--error-foreground))",
				},
				destructive: {
					DEFAULT: "hsl(var(--destructive))",
					foreground: "hsl(var(--destructive-foreground))",
				},
				muted: {
					DEFAULT: "hsl(var(--muted))",
					foreground: "hsl(var(--muted-foreground))",
				},
				accent: {
					DEFAULT: "hsl(var(--accent))",
					foreground: "hsl(var(--accent-foreground))",
				},
				popover: {
					DEFAULT: "hsl(var(--popover))",
					foreground: "hsl(var(--popover-foreground))",
				},
				card: {
					DEFAULT: "hsl(var(--card))",
					foreground: "hsl(var(--card-foreground))",
				},
			},
			borderRadius: {
				lg: "var(--radius)",
				md: "calc(var(--radius) - 2px)",
				sm: "calc(var(--radius) - 4px)",
			},
			keyframes: {
				clipPathNavMenu: {
					"0%": { opacity: "0" },
					"100%": {
						transform: "rotateX(0deg) scale(1)",
						top: "4.5em",
						opacity: "1",
					},
				},
				accordionDown: {
					from: { height: "0" },
					to: { height: "var(--primitive-collapsible-content-height)" },
				},
				accordionUp: {
					from: { height: "var(--primitive-collapsible-content-height)" },
					to: { height: "0" },
				},
				trackToastDuration: {
					"0%": { transform: "scaleX(1)" },
					"100%": { transform: "scaleX(0)" },
				},
			},
			animation: {
				clipPathNavMenu: "clipPathNavMenu 0.25s ease-out 1 forwards",
				trackToastDuration: "trackToastDuration ease-in-out forwards",
			},
		},
	},
	plugins: [require("tailwindcss-animate")],
};
