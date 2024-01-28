// src/components/icons/index.ts

// Import all SVG files in the 'icons' folder dynamically
const svgFilesContext = import.meta.glob('./*.svg');

// Create an object to store the imported icons
const icons: Record<string, any> = {};

// Dynamically import each SVG file and add it to the 'icons' object
Object.keys(svgFilesContext).forEach((key: string) => {
  const iconName = key.replace(/^.+\/([^/]+)\.svg$/, '$1');
  icons[iconName] = () => svgFilesContext[key]();
});

// Export the 'icons' object
export { icons };

