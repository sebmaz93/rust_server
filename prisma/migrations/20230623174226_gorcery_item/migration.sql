-- CreateTable
CREATE TABLE "Grocery_Item" (
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "quantity" INTEGER NOT NULL,

    CONSTRAINT "Grocery_Item_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "Grocery_Item_name_key" ON "Grocery_Item"("name");
