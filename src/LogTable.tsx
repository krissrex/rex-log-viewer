import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import LogDetails from "./LogDetails";

export default function LogTable() {
  const [logBuffer, setLogBuffer] = useState("");

  // Fetch log buffer every second
  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        setLogBuffer(await invoke("get_log_buffer", {}));
      } catch (error) {
        console.error("Failed to get log buffer:", error);
      }
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  return (
    <Table>
      <TableCaption>Logs can be sent to 127.0.0.1:54560</TableCaption>
      <TableHeader>
        <TableRow>
          <TableHead>Details</TableHead>
          <TableHead className="w-[100px]">Time</TableHead>
          <TableHead>Message</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {logBuffer.split("\n").map((log) => {
          try {
            const msg = JSON.parse(log);

            /*
            - ERROR: 40000
            - WARN: 30000
            - INFO: 20000
            - DEBUG: 10000
            - TRACE: 0
            */
            const level = msg["level_value"];
            const levelColor =
              level >= 40000 ? "red" : level >= 30000 ? "orange" : undefined;

            return (
              <TableRow style={{ borderColor: levelColor }}>
                <TableCell>
                  <LogDetails className="" log={log} />
                </TableCell>
                <TableCell className="text-right font-normal">
                  <span style={{ color: levelColor }}>{msg["@timestamp"]}</span>
                </TableCell>
                <TableCell className="font-medium">{msg["message"]}</TableCell>
              </TableRow>
            );
          } catch {
            return (
              <TableRow>
                <TableCell>
                  <LogDetails log={log} />
                </TableCell>
                <TableCell></TableCell>
                <TableCell>{log}</TableCell>
              </TableRow>
            );
          }
        })}
      </TableBody>
    </Table>
  );
}
