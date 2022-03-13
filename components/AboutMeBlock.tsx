import {useState} from "react";
import "@uiw/react-md-editor/markdown-editor.css";
import "@uiw/react-markdown-preview/markdown.css";

import dynamic from "next/dynamic";
import {CodeIcon, EyeIcon} from "@heroicons/react/solid";

import defaultText from '../data/AboutMe.md'

const AboutMeEditor = dynamic(() => import('./AboutMeEditor'),
    {ssr: false})

const md = require('markdown-it')({
    html: false,
    xhtmlOut: false,
    linkify: true,
    typographer: true,
});

export default function AboutMeBlock() {
    const [text, setText] = useState<string>(defaultText);
    const [isEditing, setIsEditing] = useState<boolean>(false);

    return (
        <div className="flex-grow rounded-2xl bg-gray-900 border border-gray-700 h-full w-full flex flex-col">
            <div className="h-20 bg-gray-800 rounded-t-2xl flex flex-row">
                <button
                    className={"flex flex-row justify-center w-32 md:w-40 text-lg md:text-xl font-bold p-2 rounded-tl-2xl" + (!isEditing ? " bg-gray-900" : "")}
                    onClick={() => setIsEditing(false)}>
                    <EyeIcon className="h-4 md:h-6 w-4 md:w-6 my-auto mr-2"/>
                    <p className="my-auto">Preview</p>
                </button>
                <button
                    className={"flex flex-row justify-center w-32 md:w-40 text-lg md:text-xl rounded-tr-2xl sm:rounded-tr-none font-bold p-2" + (isEditing ? " bg-gray-900" : "")}
                    onClick={() => setIsEditing(true)}>
                    <CodeIcon className="h-6 w-6 my-auto mr-2"/>
                    <p className="my-auto">Edit File</p>
                </button>
            </div>
            <div className="flex-grow flex flex-row">
                <div className="flex-grow p-4 pb-0 flex flex-col">
                    {
                        isEditing ?
                            <AboutMeEditor text={text} setText={setText}/>
                            :
                            <div className="flex-grow overflow-x-auto">
                                <div className="prose prose-invert" dangerouslySetInnerHTML={{__html: md.render(text)}}/>
                            </div>
                    }
                </div>
            </div>
        </div>
    )
}
