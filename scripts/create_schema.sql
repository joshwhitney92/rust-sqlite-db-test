drop table if exists RemoteJobCategory;
create table RemoteJobCategory(
    PKRemoteJobCategory Integer primary key autoincrement,
    Category Text(500) NOT NULL
);

drop table if exists RemoteJobs;
create table RemoteJobs(
    PKRemoteJobs Integer primary key autoincrement,
    Name Text(50) NOT NULL,
    Url Text(500) NOT NULL,
    FKCategory Integer NOT NULL,
    FOREIGN KEY (FKCategory) REFERENCES RemoteJobCategory (PKRemoteJobCategory)
);

insert into RemoteJobCategory ("Category") VALUES (
    "Test Category"
);

insert into RemoteJobs (
    "Name",
    "Url",
    "FKCategory"
) values
("Test 1", "test1@test.com", 1),
("Test 2", "test2@test.com", 1),
("Test 3", "test3@test.com", 1);

-- -- This should not work -- and it DOESN'T!!
-- insert into RemoteJobs (
--     "Name",
--     "Url",
--     "FKCategory"
-- ) values
-- ("Test 1", "test1@test.com", 2);




select *
from remotejobs;

select *
from remotejobcategory;
