import { useContext } from "react";
import { SettingsContext } from "../main";

interface PageProps {
  children: string | JSX.Element | JSX.Element[];
}

export default function Page(props: PageProps) {
  const { theme } = useContext(SettingsContext);
  const className = theme === "dark" ? "theme-dark" : "theme-light";
  return <div className={className}>{props.children}</div>;
}
