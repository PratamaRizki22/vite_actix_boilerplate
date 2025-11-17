-- Update passwords with correct bcrypt hashes
UPDATE users SET password = '$2b$12$0Zx.ljkaLCLjNWvjkbkQ7O3oZ7fnk8eH43XlKl8V7YK14m7n0iquC' WHERE username = 'admin';
UPDATE users SET password = '$2b$12$SUtyKJ30sGAXZejgtlhbKeGZ.POX5lPDtIF0mMNWbPc9KhIb8oJ4q' WHERE username = 'user1';
