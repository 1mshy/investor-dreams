# Theme Standardization Guide

## Overview
The app now uses a standardized dark theme throughout all components. Text colors are consistently white or light gray on dark backgrounds.

## Theme Configuration

### Color Palette
All colors are defined in `/src/app/theme/standardTheme.js`:

- **Primary Brand**: `#0080b3` (blue)
- **Background**: `#1e293b` (dark slate)
- **Text Colors**:
  - Primary: `#ffffff` (white)
  - Secondary: `#e2e8f0` (light gray)
  - Muted: `#94a3b8` (medium gray)
  - Disabled: `#64748b` (darker gray)

### Usage

#### CSS Variables
Use CSS variables in your stylesheets:
```css
.my-element {
  color: var(--text-primary);
  background-color: var(--background-color);
}
```

#### Tailwind Classes
Use Tailwind utility classes:
```jsx
<div className="text-text-primary bg-background">
  Content here
</div>
```

#### MUI Components
The theme is automatically applied to all MUI components via ThemeProvider:
```jsx
import { theme } from './app/mui/theme';
import { ThemeProvider } from '@mui/material';

<ThemeProvider theme={theme}>
  <YourComponents />
</ThemeProvider>
```

#### Direct Theme Import
For custom styled components:
```jsx
import { colors, themeStyles } from './app/theme/standardTheme';

const MyComponent = styled('div')({
  color: colors.text.primary,
  backgroundColor: colors.background.default,
});
```

## Key Changes Made

1. **Installed Tailwind CSS** with custom configuration
2. **Created standardized theme** in `/src/app/theme/standardTheme.js`
3. **Updated MUI theme** to use consistent colors
4. **Fixed hardcoded colors** in:
   - Analysis.jsx (replaced `#666` with CSS variable)
   - SearchBarTop.jsx (now uses theme colors)
   - globals.css (standardized to dark theme)
   - Homepage.css (uses CSS variables)

## Best Practices

1. **Always use theme colors** - Never hardcode colors like `#666` or `white`
2. **Use CSS variables** for styling in CSS files
3. **Use Tailwind classes** for quick styling in JSX
4. **Use theme object** for MUI styled components
5. **Maintain consistency** - All text should be readable on dark backgrounds

## Files Modified

- `tailwind.config.js` (created)
- `postcss.config.js` (created)
- `src/app/theme/standardTheme.js` (created)
- `src/app/css/globals.css` (updated)
- `src/app/css/Homepage.css` (updated)
- `src/app/mui/theme.js` (updated)
- `src/app/pages/Analysis.jsx` (fixed hardcoded colors)
- `src/components/searching/SearchBarTop.jsx` (fixed hardcoded colors)
