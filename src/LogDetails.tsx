import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@/components/ui/sheet";

export default function LogDetails() {
  return (
    <Sheet>
      <SheetTrigger>Open Details</SheetTrigger>
      <SheetContent>
        <SheetHeader>
          <SheetTitle>Log Details</SheetTitle>
          <SheetDescription>
            This is a description of the log details.
          </SheetDescription>
        </SheetHeader>
      </SheetContent>
    </Sheet>
  );
}
