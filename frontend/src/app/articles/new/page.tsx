"use client";

import { useState, useTransition } from "react";
import { useRouter } from "next/navigation";
import {
  Heading,
  FormControl,
  FormLabel,
  Input,
  Textarea,
  Button,
} from "../../common/components";
import { TagsInput } from "../[slug]/TagsInput";
import { Badge } from "../[slug]/Badg";
import clsx from 'clsx';

type TagsInputProps = React.ComponentPropsWithoutRef<'input'> & {
  isError?: boolean
  tags: string[]
  onChangeTags?: (tags: string[]) => void
}

export default function CreateArticle() {
  const router = useRouter();
  const [title, setTitle] = useState("");
  const [content, setContent] = useState("");
  const [tags, setTags] = useState<string[]>([]);
  const [loading, setLoading] = useState(false);
  const [isPending, startTransition] = useTransition();

  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    setLoading(true);
    await fetch("/api/articles", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ title, content, tags }), // タグも送信する
    });
    router.push("/");
    startTransition(() => {
      router.refresh();
    });
  };


  return (
    <div>
      <Heading mb={4}>Create Article</Heading>

      <form onSubmit={handleSubmit}>
        <FormControl>
          <FormLabel>タイトル</FormLabel>
          <Input value={title} onChange={(e) => setTitle(e.target.value)} />

          <FormLabel>本文</FormLabel>
          <Textarea
            value={content}
            onChange={(e) => setContent(e.target.value)}
          />

          <FormLabel>タグ</FormLabel>
          <TagsInput tags={tags} onChangeTags={(newTags) => { setTags(newTags)} } /> 

          <Button
            type="submit"
            color="white"
            bg="orange.400"
            isLoading={loading || isPending}
            mt={4}
          >
            作成
          </Button>
        </FormControl>
      </form>
    </div>
  );
}

