CREATE TABLE rooms(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   host_id uuid NOT NULL
      REFERENCES hosts (id),
   name TEXT NOT NULL,
   description TEXT NOT NULL,
   number_of_beds SMALLINT NOT NULL CHECK(number_of_beds > 0)
);

-- There is auto generated name for check above
-- {table}_{column}_check  == rooms_number_of_beds_check

-- Drop the check for rollback purpose
/*
ALTER TABLE table_name
DROP CONSTRAINT constraint_name;
 */

 -- Or add to existing table
 /*
ALTER TABLE table_name
ADD CONSTRAINT constraint_name CHECK (condition);
 */
