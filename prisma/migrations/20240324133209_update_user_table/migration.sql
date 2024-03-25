/*
  Warnings:

  - Added the required column `test2` to the `User` table without a default value. This is not possible if the table is not empty.
  - Added the required column `test3` to the `User` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE `User` ADD COLUMN `test2` VARCHAR(191) NOT NULL,
    ADD COLUMN `test3` VARCHAR(191) NOT NULL;
