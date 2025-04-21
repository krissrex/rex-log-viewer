import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "@/components/ui/input";
import LogTable from "./LogTable";
import { ThemeProvider } from "@/components/theme-provider";
import { ModeToggle } from "@/components/mode-toggle";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <main className="flex flex-col items-center min-h-screen p-4">
        <ModeToggle className="absolute right-4 top-4" />
        <h1 className="self-start scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl">
          Log Viewer
        </h1>
        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            greet();
          }}
        >
          <Input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            onKeyUp={(e) => {
              if (e.key === "Enter") {
                e.preventDefault();
                greet();
              }
            }}
            placeholder="Search"
          />
        </form>
        <p>{greetMsg}</p>
        <LogTable />
      </main>
    </ThemeProvider>
  );
}

export default App;
