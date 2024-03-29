import {
    Card,
    CardHeader,
    CardBody,
    CardFooter,
    Heading,
    Text,
} from "./common/components";
import NextLink from "next/link";
import { Article } from "./types";
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import remarkMath from 'remark-math';
import rehypeKatex from 'rehype-katex';
import './articles/[slug]/markdown.css';
import 'katex/dist/katex.min.css';

export default function ArticleCard({ article }: { article: Article }) {
    const formattedDate = new Date(article.createdAt).toLocaleDateString(
        "ja-JP",
        {
            year: "numeric",
            month: "long",
            day: "numeric",
        }
    );
    return (
        <Card
            as={"li"}
            _hover={{
                boxShadow: "xl",
            }}
            minW="100%"
        >
            <NextLink href={`/articles/${article.slug}`}>
                <CardHeader>
                    <Heading size="md">{article.title}</Heading>
                </CardHeader>
                <CardBody>
                    <ReactMarkdown className={"markdown"} remarkPlugins={[remarkMath, remarkGfm]} rehypePlugins={[rehypeKatex]}>
                    {article.content.substring(0,200)}
                    </ReactMarkdown>
                </CardBody>
                <CardFooter>
                    <Text fontSize="sm" color="gray.600">
                        {formattedDate}
                    </Text>
                </CardFooter>
            </NextLink>
        </Card>
    );
}