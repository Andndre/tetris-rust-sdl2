import { useContext } from "react";
import { SettingsContext } from "../main";

interface PageProps {
  children: string | JSX.Element | JSX.Element[];
}

// page with automatic theme
export default (props: PageProps) => {
  const { theme } = useContext(SettingsContext);
  const className = theme === "dark" ? "theme-dark" : "theme-light";
  return <div className={className}>{props.children}</div>;
};
