"use client";
import { FaGithub, FaGitlab, FaLinkedin } from 'react-icons/fa';
import { motion } from 'framer-motion';

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col justify-center items-center bg-gray-100 text-gray-900">
       <motion.header
        className="text-center mb-10"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 1 }}
      >
        <h1 className="text-4xl font-bold text-gray-800 mb-4">Welcome to My Portfolio</h1>
        <p className="text-xl text-gray-600">Connect with me through the platforms below:</p>
      </motion.header>

      <div className="flex space-x-8">
        <a
          href="https://github.com/your-username"
          target="_blank"
          rel="noopener noreferrer"
          className="flex flex-col items-center text-gray-700 hover:text-gray-900 transition duration-300"
        >
          <FaGithub size={50} />
          <span className="mt-2 text-lg">GitHub</span>
        </a>

        <a
          href="https://gitlab.com/your-username"
          target="_blank"
          rel="noopener noreferrer"
          className="flex flex-col items-center text-gray-700 hover:text-gray-900 transition duration-300"
        >
          <FaGitlab size={50} />
          <span className="mt-2 text-lg">GitLab</span>
        </a>

        <a
          href="https://www.linkedin.com/in/your-username/"
          target="_blank"
          rel="noopener noreferrer"
          className="flex flex-col items-center text-gray-700 hover:text-gray-900 transition duration-300"
        >
          <FaLinkedin size={50} />
          <span className="mt-2 text-lg">LinkedIn</span>
        </a>
      </div>

      <footer className="mt-20 text-gray-500 text-sm">
        <p>Designed with 💙 using Next.js and Tailwind CSS</p>
      </footer>
    </div>
  );
}
