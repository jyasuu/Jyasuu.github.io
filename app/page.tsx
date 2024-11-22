"use client";
import { FaGithub, FaGitlab, FaLinkedin, FaCode } from 'react-icons/fa';
import { motion } from 'framer-motion';

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col justify-center items-center bg-gray-100 text-gray-900 p-6">
      <motion.header
        className="text-center mb-10"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ duration: 1 }}
      >
        <h1 className="text-4xl sm:text-5xl font-bold text-gray-800 mb-4">Welcome to My Portfolio</h1>
        <p className="text-lg sm:text-xl text-gray-600">Connect with me through the platforms below:</p>
      </motion.header>

      <div className="flex flex-wrap justify-center gap-8">
        {/* GitHub */}
        <a
          href="https://github.com/jyasuu"
          target="_blank"
          rel="noopener noreferrer"
          className="flex flex-col items-center text-gray-700 hover:text-gray-900 transition duration-300"
        >
          <motion.div
            whileHover={{ scale: 1.1 }}
            whileTap={{ scale: 0.9 }}
          >
            <FaGithub size={50} />
          </motion.div>
          <span className="mt-2 text-lg">GitHub</span>
        </a>

        {/* GitLab */}
        <a
          href="https://gitlab.com/jyasuu"
          target="_blank"
          rel="noopener noreferrer"
          className="flex flex-col items-center text-gray-700 hover:text-gray-900 transition duration-300"
        >
          <motion.div
            whileHover={{ scale: 1.1 }}
            whileTap={{ scale: 0.9 }}
          >
            <FaGitlab size={50} />
          </motion.div>
          <span className="mt-2 text-lg">GitLab</span>
        </a>

        {/* LinkedIn */}
        <a
          href="https://www.linkedin.com/in/zi-xuan-yen-403b47133/"
          target="_blank"
          rel="noopener noreferrer"
          className="flex flex-col items-center text-gray-700 hover:text-gray-900 transition duration-300"
        >
          <motion.div
            whileHover={{ scale: 1.1 }}
            whileTap={{ scale: 0.9 }}
          >
            <FaLinkedin size={50} />
          </motion.div>
          <span className="mt-2 text-lg">LinkedIn</span>
        </a>

        {/* LeetCode */}
        <a
          href="https://leetcode.com/u/jyasu/"
          target="_blank"
          rel="noopener noreferrer"
          className="flex flex-col items-center text-gray-700 hover:text-gray-900 transition duration-300"
        >
          <motion.div
            whileHover={{ scale: 1.1 }}
            whileTap={{ scale: 0.9 }}
          >
            <FaCode size={50} />
          </motion.div>
          <span className="mt-2 text-lg">LeetCode</span>
        </a>
      </div>

      <footer className="mt-16 text-gray-500 text-sm text-center">
        <p>Designed with 💙 using Next.js and Tailwind CSS</p>
      </footer>
    </div>
  );
}
