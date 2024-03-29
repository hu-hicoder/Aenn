import { Badge } from './Badg';
import clsx from 'clsx';
import { useState, useTransition } from "react";
import { useRouter } from "next/navigation";

type TagsInputProps = React.ComponentPropsWithoutRef<'input'> & {
    isError?: boolean
    tags: string[]
    onChangeTags?: (tags: string[]) => void
}

export const TagsInput: React.FC<TagsInputProps> = ({ onChangeTags, tags = [], isError, className, ...props }) => {
    const router = useRouter();
    const [inputTags, setTags] = useState<string[]>([]);
    const [loading, setLoading] = useState(false);
    const [isPending, startTransition] = useTransition();
    const [localTags, setLocalTags] = useState<string[]>(tags);

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        setLoading(true);
        await fetch("/api/articles", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(tags), 
        });
        router.push("/");
        startTransition(() => {
          router.refresh();
        });
      };

    // タグが変更されたときのハンドラー
    const handleChangeTags = (newTags: string[]) => {
        setTags(newTags);
    };

    const onClose = (i: number) => {
        const newTags = [...localTags];
        newTags.splice(i, 1);
        setLocalTags(newTags);
        onChangeTags && onChangeTags(newTags);
    }

    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.nativeEvent.isComposing) return;

        const value = e.currentTarget.value;
        if (e.key === 'Backspace' && !value.length && localTags.length > 0) {
            onClose(localTags.length - 1);
            return;
        }

        if (e.key !== 'Enter' || !value.trim()) return;
        const newTags = [...localTags, value];
        setLocalTags(newTags);
        onChangeTags && onChangeTags(newTags);
        e.currentTarget.value = '';
        e.preventDefault();
    }

    const styles = {
        default: 'border-gray-200 focus:bg-white focus:border-gray-500',
        error: 'border-red-500 focus:bg-white focus:border-gray-500'
    };
  
    return (
        <div
            className={clsx(
                'flex flex-wrap text-gray-700 border leading-tight pt-3 pb-2 px-4 rounded',
                styles[isError ? 'error' : 'default']
            )}
        >
            {localTags.map((tag, i) => {
                return (
                    <Badge key={i} size="sm" className={'mr-1 mb-1'} onClose={() => onClose(i)}>
                        {tag}
                    </Badge>
                )
            })}
            <input
                type="text"
                className={'flex-grow border-0 mb-1 outline-none'}
                onKeyDown={handleKeyDown}
                {...props}
            />
            <form onSubmit={handleSubmit}></form>
        </div>
    )
}