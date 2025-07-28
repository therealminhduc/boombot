import { Button } from "@/components/ui/button";
import { Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";

export default function ActionModal() {
    return (
        <Dialog>
            <DialogTrigger asChild>
                <Button size="sm" variant="destructive">Delete</Button>
            </DialogTrigger>

            <DialogContent className="sm:max-w-md">
                <DialogHeader>
                    <DialogTitle>Are you sure about deleting this rule?</DialogTitle>
                </DialogHeader>

                <DialogFooter className="sm:justify-start">
                    <DialogClose asChild>
                        <Button variant="secondary">Cancel</Button>
                    </DialogClose>

                    <DialogClose asChild>
                        <Button variant="destructive">Delete</Button>
                    </DialogClose>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    )
}