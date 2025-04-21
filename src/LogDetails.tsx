import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";
import { useMemo } from "react";
import { ScrollArea } from "./components/ui/scroll-area";
import { cn } from "./lib/utils";

interface LogDetailsProps {
  log?: string;
  className?: string;
}

export default function LogDetails({ log = "", className }: LogDetailsProps) {
  const logJson = useMemo(() => {
    try {
      return JSON.parse(log);
    } catch {
      return {};
    }
  }, [log]);

  return (
    <Sheet>
      <SheetTrigger className={cn("cursor-pointer", className)}>
        View
      </SheetTrigger>
      <SheetContent className="w-full max-w-full">
        <SheetHeader>
          <SheetTitle>View</SheetTitle>
          <SheetDescription>
            <table className="">
              {Object.entries(logJson).map(([key, val]) => (
                <tr className="border-1" key={key}>
                  <td className="pr-2">{key}:</td>{" "}
                  <td className="pl-2">{val}</td>
                </tr>
              ))}
            </table>
          </SheetDescription>
        </SheetHeader>
      </SheetContent>
    </Sheet>
  );
}
